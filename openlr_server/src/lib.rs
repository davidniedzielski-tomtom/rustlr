pub mod openlr_services {
    tonic::include_proto!("openlr_services");
}

pub mod decode_request;
pub mod http_map_proxy;
pub mod grpc_map_proxy;
pub mod server_context;
pub mod grpc_server_context;
pub mod common;