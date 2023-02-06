use clap::Parser;
use geo::Coord;
use mock_map_server::mock_map::MockMap;
use openlr::map_server::MapServer;
use openlr::{edge::Edge, fow::FOW, frc::FRC};
use tonic::Code;
use tonic::{transport::Server, Request, Response, Status};

#[derive(Parser, Default, Debug)]
#[clap(author = "TomTom International", version, about)]
/// Mock Map gRPC MapServer
pub struct Arguments {
    /// CSV file from which the map will be derived
    file_path: String,
    #[clap(default_value_t=String::from("[::1:8080]"),short, long)]
    /// Address to listen on (i.e. "127.0.0.1:9090")
    address: String,
}

pub mod openlr_services {
    tonic::include_proto!("openlr_services");
}

use openlr_services::{
    map_service_server::{MapService, MapServiceServer},
    Coordinate, EdgeSet, NextEdgesRequest, RadiusSearchRequest, RadiusSearchResponse,
};

use openlr_services::Edge as ProtoEdge;

fn proto_fow_from_fow(fow: &FOW) -> i32 {
    match fow {
        FOW::Undefined => 0,
        FOW::Motorway => 1,
        FOW::MultipleCarriageway => 2,
        FOW::SingleCarriageway => 3,
        FOW::Roundabout => 4,
        FOW::TrafficSquare => 5,
        FOW::SlipRoad => 6,
        FOW::Other => 7,
    }
}

fn proto_frc_from_frc(frc: &FRC) -> i32 {
    match frc {
        FRC::FRC0 => 0,
        FRC::FRC1 => 1,
        FRC::FRC2 => 2,
        FRC::FRC3 => 3,
        FRC::FRC4 => 4,
        FRC::FRC5 => 5,
        FRC::FRC6 => 6,
        FRC::FRC7 => 7,
    }
}

fn proto_edge_from_edge(e: &Edge) -> ProtoEdge {
    ProtoEdge {
        id: e.id,
        meta: e.meta.clone(),
        fow: proto_fow_from_fow(&e.fow),
        frc: proto_frc_from_frc(&e.frc),
        len: e.len,
        coords: e
            .geom
            .coords()
            .cloned()
            .map(|c| Coordinate {
                longitude: c.x,
                latitude: c.y,
            })
            .collect::<Vec<Coordinate>>(),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_env().unwrap();
    let args = Arguments::parse();
    let address = args.address.parse().unwrap();
    let map_service = MockMapService::new(&args.file_path);

    log::info!("Mock map server with source: {} initializing...", args.file_path);
    log::info!("Mock map server listening on port: {}...", args.address);

    Server::builder()
        .add_service(MapServiceServer::new(map_service))
        .serve(address)
        .await?;

    log::info!("Mock map server terminating...");

    Ok(())
}

#[derive(Debug)]
pub struct MockMapService {
    mock_map: MockMap,
}

impl MockMapService {
    pub fn new(file: &str) -> Self {
        MockMapService {
            mock_map: MockMap::new_from_csv(file),
        }
    }
}

#[tonic::async_trait]
impl MapService for MockMapService {
    async fn next_edges(
        &self,
        request: Request<NextEdgesRequest>,
    ) -> Result<Response<EdgeSet>, Status> {
        let rsp = self
            .mock_map
            .get_next_lines(request.get_ref().id, request.get_ref().meta.clone())
            .await
            .or_else(|e| Err(Status::new(Code::Internal, e.to_string())))?;
        Ok(Response::new(EdgeSet {
            edges: rsp
                .iter()
                .map(|e| proto_edge_from_edge(&e))
                .collect::<Vec<ProtoEdge>>(),
        }))
    }

    async fn radius_search(
        &self,
        request: Request<RadiusSearchRequest>,
    ) -> Result<Response<RadiusSearchResponse>, Status> {
        let rsr = request.into_inner();

        let edge_sets = self
            .mock_map
            .get_lines_near_coordinates(
                rsr.points
                    .iter()
                    .map(|c| Coord {
                        x: c.longitude,
                        y: c.latitude,
                    })
                    .collect::<Vec<Coord>>(),
                rsr.radius,
            )
            .await
            .or_else(|e| Err(Status::new(Code::Internal, e.to_string())))?
            .iter()
            .map(|v| EdgeSet { edges:
                v.iter()
                    .map(|e| proto_edge_from_edge(e))
                    .collect::<Vec<ProtoEdge>>()
            })
            .collect::<Vec<EdgeSet>>();

            Ok ( Response::new(RadiusSearchResponse { edge_sets } ) )
    }
}
