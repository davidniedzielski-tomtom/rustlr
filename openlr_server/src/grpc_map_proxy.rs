pub mod openlr_services {
    tonic::include_proto!("openlr_services");
}

use crate::grpc_map_proxy::openlr_services::{
    map_service_client::MapServiceClient, Coordinate, NextEdgesRequest,
    RadiusSearchRequest,
};

use async_trait::async_trait;
use geo::{Coord, LineString};
use openlr::{edge::Edge, errors::OpenLrErr, fow::FOW, frc::FRC, map_server::MapServer};
use reqwest::Url;
use tonic::transport::Channel;


fn edge_from_proto_edge(e: &crate::grpc_map_proxy::openlr_services::Edge) -> Edge {
    Edge {
        id: e.id,
        meta: e.meta.clone(),
        fow: FOW::from_u8(e.fow as u8),
        frc: FRC::from_u8(e.frc as u8),
        len: e.len,
        geom: LineString(
            e.coords
                .iter()
                .map(|c| Coord {
                    x: c.longitude,
                    y: c.latitude,
                })
                .collect::<Vec<Coord>>(),
        ),
    }
}

pub struct GRPCMapProxy<Channel> {
    client: MapServiceClient<Channel>,
}

impl GRPCMapProxy<Channel> {
    pub async fn new(url: Url) -> Self {
        let client = MapServiceClient::connect(url.to_string()).await.unwrap();
        GRPCMapProxy { client }
    }
}

#[async_trait]
impl MapServer for GRPCMapProxy<Channel> {
    async fn get_lines_near_coordinates(
        &self,
        points: Vec<Coord>,
        radius: u32,
    ) -> Result<Vec<Vec<Edge>>, OpenLrErr> {
        let rsr = RadiusSearchRequest {
            points: points
                .iter()
                .map(|c| Coordinate {
                        longitude: c.x,
                        latitude: c.y,
                    })
                .collect::<Vec<Coordinate>>(),
                radius
        };
        let request = tonic::Request::new(rsr);
        let mut c = self.client.clone();
        let response = c
            .radius_search(request)
            .await
            .map_err(|s| OpenLrErr::RadiusSearchError(s.to_string()))?;
        Ok(response
            .into_inner()
            .edge_sets
            .iter()
            .map(|es| {
                es.edges
                    .iter()
                    .map(|e| edge_from_proto_edge(e))
                    .collect::<Vec<Edge>>()
            })
            .collect::<Vec<Vec<Edge>>>())
    }

    /// Returns a set of lines which follows this line in the same direction. The set of lines
    /// is equal to the set of outgoing lines of the end node of this line.
    async fn get_next_lines(&self, id: i64, meta: String) -> Result<Vec<Edge>, OpenLrErr> {
        let ner = NextEdgesRequest { id, meta };
        let request = tonic::Request::new(ner);
        let mut c = self.client.clone();
        let response = c
            .next_edges(request)
            .await
            .map_err(|s| OpenLrErr::NextEdgeError(s.to_string()))?;
        Ok(response
            .into_inner()
            .edges
            .iter()
            .map(|e| edge_from_proto_edge(e))
            .collect::<Vec<Edge>>())
    }
}
