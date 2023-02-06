use clap::Parser;
use std::collections::hash_map::Entry;
use std::sync::Arc;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use openlr::log::LogLevel;
use openlr_server::decode_request::DecodeRequest;
use openlr_server::grpc_map_proxy::GRPCMapProxy;
use openlr_server::http_map_proxy::HttpMapProxy;
use openlr_server::server_context::ServerContext;
use reqwest::Url;

#[derive(Parser, Default, Debug)]
#[clap(author = "TomTom International", version, about)]
/// Mock Map gRPC MapServer
pub struct Arguments {
    #[clap(default_value_t=String::from("[::1]:8080"),short, long)]
    /// Interface address to bind to (i.e. 127.0.0.1:8080)
    address: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn decode(
    params: web::Json<DecodeRequest>,
    context: web::Data<ServerContext>,
) -> impl Responder {
    let url = match Url::parse(&params.url) {
        Ok(url) => url,
        _ => {
            return HttpResponse::BadRequest().body("Invalid URL syntax");
        }
    };

    let log_level = match params.log_level.to_ascii_lowercase().as_str() {
        "trace" => LogLevel::Trace,
        "debug" => LogLevel::Debug,
        "info" => LogLevel::Info,
        "warn" => LogLevel::Warn,
        "error" => LogLevel::Error,
        "fatal" => LogLevel::Fatal,
        _ => return HttpResponse::BadRequest().body("Invalid log evel.  Must be one of: {trace,debug,info,warn,error,fatal}"),
    };

    let id = match params.id.parse::<u64>() {
        Ok(v) => v,
        _ => return HttpResponse::BadRequest().body("Unable to parse aid as u64"),
    };

    // Retreive a cached Map for the chosen URL, or else create a new one
    let mut mdbs = context.mdbs.lock().unwrap();

    let mdb = match mdbs.entry(url) {
        Entry::Occupied(e) => (*e.into_mut()).clone(),
        Entry::Vacant(e) => {
            let dup = e.key().clone();
            match dup.scheme() {
                "http" | "https" => (*e.insert(Arc::new(HttpMapProxy::new(dup)))).clone(),
                "grpc" | "grpcs" => {
                    (*e.insert(Arc::new(GRPCMapProxy::new(dup).await))).clone()
                }
                _ => return HttpResponse::BadRequest().body("Unknown URL scheme"),
            }
        }
    };

    // Drop the lock on the map hash early so that later mapnics do not poison the mutex
    drop(mdbs);

    // Retreive the request paramter set from the server context
    let parameter_set = match context.get_param_set(&params.params_key) {
        Some(x) => x.clone(),
        _ => return HttpResponse::BadRequest().body("Unknown parameter set"),
    };

    let res = openlr::decode_binary(
        &params.openlr_code,
        id,
        mdb.as_ref(),
        &parameter_set,
        log_level,
    )
    .await;
    HttpResponse::Ok().body(serde_json::to_string(&res).unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    simple_logger::init_with_env().unwrap();
    let args = Arguments::parse();
    log::info!("OpenLR server initializing...");
    log::info!("Binding to {}", args.address);

    let context = web::Data::new(ServerContext::new());
    HttpServer::new(move || {
        App::new()
            .app_data(context.clone())
            .service(hello)
            .service(echo)
            .route("/decode", web::post().to(decode))
    })
    .bind(args.address)?
    .run()
    .await
}

#[cfg(test)]
mod test {

    use crate::main;
    #[test]
    fn test_main() {
        main().unwrap()
    }

    #[test]
    fn test_fmt() {
        struct Logger {
            enabled: bool,
        }

        impl Logger {
            fn log<F>(&self, f: F)
            where
                F: Fn() -> String,
            {
                if self.enabled {
                    println!("{}", f());
                }
            }
        }

        let l1 = Logger { enabled: false };
        let l2 = Logger { enabled: true };

        l1.log(|| {
            println!("Executing");
            format!("This is a message for logger: {}", 1)
        });
        l2.log(|| {
            println!("Executing");
            format!("This is a message for logger: {}", 2)
        });
    }
}
