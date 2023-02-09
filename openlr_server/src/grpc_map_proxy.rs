use async_trait::async_trait;
use crate::common::edge_from_proto_edge;
use geo::Coord;
use openlr::{edge::Edge, errors::OpenLrErr, map::Map};
use reqwest::Url;
use tonic::transport::Channel;

use crate::openlr_services::{
    map_agent_client::MapAgentClient, Coordinate, NextEdgesRequest,
    NearbyEdgesRequest,
};

pub struct GRPCMapProxy<Channel> {
    client: MapAgentClient<Channel>,
}

impl GRPCMapProxy<Channel> {
    pub async fn new(url: Url) -> Self {
        let client = MapAgentClient::connect(url.to_string()).await.unwrap();
        GRPCMapProxy { client }
    }
}

#[async_trait]
impl Map for GRPCMapProxy<Channel> {
    async fn get_nearby_edges(
        &self,
        points: Vec<Coord>,
        radius: u32,
    ) -> Result<Vec<Vec<Edge>>, OpenLrErr> {
        let rsr = NearbyEdgesRequest {
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
            .get_nearby_edges(request)
            .await
            .map_err(|s| OpenLrErr::NearbyEdgesError(s.to_string()))?;
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
    async fn get_next_edges(&self, id: i64, meta: String) -> Result<Vec<Edge>, OpenLrErr> {
        let ner = NextEdgesRequest { id, meta };
        let request = tonic::Request::new(ner);
        let mut c = self.client.clone();
        let response = c
            .get_next_edges(request)
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
