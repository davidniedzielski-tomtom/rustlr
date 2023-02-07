use geo::Coord;
use openlr::{map::Map, edge::Edge, errors::OpenLrErr};
use async_trait::async_trait;
use reqwest::Url;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct RadiusSearchRequestPoint {
    pub lon: f64,
    pub lat: f64,
    pub radius: u32,
}

#[derive(Debug, Serialize)]
pub struct RadiusSearchRequest {
    pub points: Vec<RadiusSearchRequestPoint>
}

impl RadiusSearchRequest {
    pub fn new(points: &Vec<Coord>, radius: u32) -> Self {
        RadiusSearchRequest { 
            points:points.iter().map(|c| 
                RadiusSearchRequestPoint{ 
                    lon: c.x, 
                    lat: c.y, 
                    radius 
                }
            )
            .collect::<Vec<RadiusSearchRequestPoint>>() 
        }
    }
}

#[derive(Debug, Serialize)]
pub struct RadiusSearchResponse (Vec<Edge>);

#[derive(Debug, Serialize)]
pub struct NextEdgeRequest {
    pub id: u64,
    pub meta: String
}

impl NextEdgeRequest {
    pub fn new(id: u64, meta: String) -> Self {
        NextEdgeRequest {
            id,
            meta
        }
    }
}

#[derive(Debug, Serialize)]
pub struct NextEdgeResponse (Vec<Edge>);

pub struct HttpMapProxy {
    url: Url,
}

impl HttpMapProxy {
    pub fn new(url: Url) -> Self {
        HttpMapProxy {url}
    }
}

#[async_trait]
impl Map for HttpMapProxy {
    async fn get_nearby_edges(
        &self,
        points: Vec<Coord>,
        radius: u32,
    ) -> Result<Vec<Vec<Edge>>, OpenLrErr> {
        todo!()
    }

    /// Returns a set of lines which follows this line in the same direction. The set of lines
    /// is equal to the set of outgoing lines of the end node of this line.
    async fn get_next_edges(
        &self,
        id: i64,
        meta: String,
    ) -> Result<Vec<Edge>, OpenLrErr> {
        todo!()
    }
}