use clap::Parser;
use openlr_server::common::proto_edge_from_edge;
use openlr::location::Location;
use openlr::log::{LogEntry, LogLevel};
use openlr::request_result::RequestResult;
use openlr_server::grpc_map_proxy::GRPCMapProxy;
use openlr_server::grpc_server_context::GrpcServerContext;
use openlr_server::openlr_services::decoder_server::Decoder;
use openlr_server::openlr_services::{BinaryDecodeRequest, BinaryDecodeResponse, binary_decode_response, DecodeError};
use openlr_server::openlr_services::{LogMessage, OffsetRange};
use reqwest::Url;
use std::collections::hash_map::Entry;
use std::collections::VecDeque;
use std::sync::Arc;
use std::{error::Error, io::ErrorKind, pin::Pin};
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::codegen::futures_core::Stream;
use tonic::Streaming;
use tonic::{transport::Server, Request, Response, Status};

use openlr_server::openlr_services::decoder_server::DecoderServer;

#[derive(Parser, Default, Debug)]
#[clap(author = "TomTom International", version, about)]
/// Mock Map gRPC MapServer
pub struct Arguments {
    #[clap(default_value_t=String::from("[::1:8080]"),short, long)]
    /// Address to listen on (i.e. "127.0.0.1:9090")
    address: String,
}

fn match_for_io_error(err_status: &Status) -> Option<&std::io::Error> {
    let mut err: &(dyn Error + 'static) = err_status;

    loop {
        if let Some(io_err) = err.downcast_ref::<std::io::Error>() {
            return Some(io_err);
        }

        // h2::Error do not expose std::io::Error with `source()`
        // https://github.com/hyperium/h2/pull/462
        if let Some(h2_err) = err.downcast_ref::<h2::Error>() {
            if let Some(io_err) = h2_err.get_io() {
                return Some(io_err);
            }
        }

        err = match err.source() {
            Some(err) => err,
            None => return None,
        };
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_env().unwrap();
    let args = Arguments::parse();
    let address = args.address.parse().unwrap();
    let decode_service = DecoderService::new();

    log::info!("Decoder server initializing...",);
    log::info!("Decoder server listening on port: {}...", args.address);

    Server::builder()
        .add_service(DecoderServer::new(decode_service))
        .serve(address)
        .await?;

    log::info!("Decoder server terminating...");

    Ok(())
}

pub struct DecoderService {
    context: GrpcServerContext,
}

fn log_message_to_proto(le: LogEntry) -> LogMessage {
    LogMessage {
        level: match le.level {
            LogLevel::Trace => 0,
            LogLevel::Debug => 1,
            LogLevel::Info => 3,
            LogLevel::Warn => 4,
            LogLevel::Error => 5,
            _ => 6,
        },
        msg: le.txt,
    }
}

fn build_decode_response(res: &RequestResult<Location>) -> BinaryDecodeResponse {
    // FIXME: avoid the clone()
    let logmsgs = VecDeque::from(res.log.clone())
        .drain(..)
        .map(|le| log_message_to_proto(le))
        .collect::<Vec<LogMessage>>();

    match &res.result {
        Err(e) => BinaryDecodeResponse {
            id: res.id,
            elapsed_secs: res.elapsed.as_secs(),
            elapsed_nanosecs: res.elapsed.subsec_nanos(),
            log: logmsgs,
            decoding_result: Some(
                binary_decode_response::DecodingResult::DecodeError(
                    DecodeError {
                        reason: e.to_string(),
                    },
                ),
            ),
        },
        Ok(Location::Line(line)) => BinaryDecodeResponse {
            id: res.id,
            elapsed_secs: res.elapsed.as_secs(),
            elapsed_nanosecs: res.elapsed.subsec_nanos(),
            log: logmsgs,
            decoding_result: Some(
                binary_decode_response::DecodingResult::LineLocation(
                    openlr_server::openlr_services::LineLocation {
                        edge: line.edges.iter().map(|e| proto_edge_from_edge(e)).collect::<Vec<openlr_server::openlr_services::Edge>>(),
                        pos_off: match line.p_off {
                            Some((lb, ub)) => Some(OffsetRange { lb, ub }),
                            _ => None,
                        },
                        neg_off: match line.n_off {
                            Some((lb, ub)) => Some(OffsetRange { lb, ub }),
                            _ => None,
                        },
                    },
                ),
            ),
        },
        // TODO: Add PointALongLine
        Ok(_) => todo!()
    }
}

impl DecoderService {
    pub fn new() -> Self {
        DecoderService {
            context: GrpcServerContext::new(),
        }
    }
    async fn do_decode(&self, req: &BinaryDecodeRequest) -> BinaryDecodeResponse {
        // Retreive a cached Map for the chosen URL, or else create a new one

        let url = match Url::parse(&req.agent_name) {
            Ok(u) => u,
            // TODO: Deal with Url parse failure
            _ => todo!(),
        };

        let log_level = match req.logging_level {
            0 => LogLevel::Trace,
            1 => LogLevel::Debug,
            2 => LogLevel::Info,
            3 => LogLevel::Warn,
            4 => LogLevel::Error,
            _ => LogLevel::Fatal,
        };

        let mut mdbs = self.context.mdbs.lock().await;
        let mdb = match mdbs.entry(url) {
            Entry::Occupied(e) => (*e.into_mut()).clone(),
            Entry::Vacant(e) => {
                let dup = e.key().clone();
                (*e.insert(Arc::new(GRPCMapProxy::new(dup).await))).clone()
            }
        };

        // Drop the lock on the map hash early so that any future panics do not poison the mutex
        drop(mdbs);

        // Retreive the request parameter set from the server context
        let parameter_set = match self
            .context
            .get_param_set(&req.decoding_parameter_set)
            .await
        {
            Some(x) => x.clone(),
            // TODO: Deal with unknown parameter set
            _ => todo!(),
        };

        let res =
            openlr::decode_binary(&req.code, req.id, mdb.as_ref(), &parameter_set, log_level).await;

        build_decode_response(&res)
    }
}

type ResponseStream = Pin<Box<dyn Stream<Item = Result<BinaryDecodeResponse, Status>> + Send>>;

#[tonic::async_trait]
impl Decoder for DecoderService {
    type DecodeBinaryStream = ResponseStream;
    async fn decode_binary(
        &self,
        req: Request<Streaming<BinaryDecodeRequest>>,
    ) -> Result<Response<Self::DecodeBinaryStream>, Status> {
        let mut in_stream = req.into_inner();
        let (tx, rx) = mpsc::channel(128);
        let out_stream = ReceiverStream::new(rx);

        while let Some(result) = in_stream.next().await {
            match result {
                Ok(bin_dec_req) => tx
                    .send(Ok(self.do_decode(&bin_dec_req).await))
                    .await
                    .expect("working rx"),
                Err(err) => {
                    if let Some(io_err) = match_for_io_error(&err) {
                        if io_err.kind() == ErrorKind::BrokenPipe {
                            // here you can handle special case when client
                            // disconnected in unexpected way
                            eprintln!("\tclient disconnected: broken pipe");
                            break;
                        }
                    }

                    match tx.send(Err(err)).await {
                        Ok(_) => (),
                        Err(_err) => break, // response was droped
                    }
                }
            }
        }

        Ok(Response::new(
            Box::pin(out_stream) as Self::DecodeBinaryStream
        ))
    }
}
