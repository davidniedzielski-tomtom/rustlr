use crate::binary_header::{AreaFlag, BinaryHeader};

#[derive(Debug)]
pub enum LocationType {
    Line = 0,
    GeoCoordinate = 1,
    PointAlongLine = 2,
    PoiWithAccessPoint = 3,
    Circle = 4,
    Rectangle = 5,
    Grid = 6,
    Polygon = 7,
    ClosedLine = 8,
    Unknown = 9,
}

impl From<&Vec<u8>> for LocationType {
    fn from(value: &Vec<u8>) -> Self {
        let len = value.len();
        if len == 0 {
            LocationType::Unknown
        } else {
            let bh = BinaryHeader::new(value.get(0).unwrap().to_owned());
            match (bh.is_point(), bh.area_flag(), bh.has_attrs()) {
                (false, AreaFlag::CircleOrNoAreaLocation, true) => LocationType::Line,
                (true, AreaFlag::CircleOrNoAreaLocation, false) => LocationType::GeoCoordinate,
                (true, AreaFlag::CircleOrNoAreaLocation, true) if (len == 16 || len == 17) => {
                    LocationType::PointAlongLine
                }
                (true, AreaFlag::CircleOrNoAreaLocation, true) if (len == 20 || len == 21) => {
                    LocationType::PoiWithAccessPoint
                }
                (false, AreaFlag::CircleOrNoAreaLocation, false) => LocationType::Circle,
                (false, AreaFlag::RectangleOrGrid, false) if (len == 11 || len == 13) => {
                    LocationType::Rectangle
                }
                (false, AreaFlag::RectangleOrGrid, false) if (len == 15 || len == 17) => {
                    LocationType::Grid
                }
                (false, AreaFlag::Polygon, false) => LocationType::Polygon,
                (false, AreaFlag::ClosedLine, true) => LocationType::ClosedLine,
                _ => LocationType::Unknown,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line() {
        let v = vec![11u8];
        match LocationType::from(&v) {
            LocationType::Line => assert!(true),
            _ => assert!(false),
        }
    }
    #[test]
    fn test_geocoordinate() {
        let v = vec![0x23u8];
        match LocationType::from(&v) {
            LocationType::GeoCoordinate => assert!(true),
            _ => assert!(false),
        }
    }
    #[test]
    fn test_point_along_line() {
        let v0 = vec![0x2bu8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        match LocationType::from(&v0) {
            LocationType::PointAlongLine => assert!(true),
            _ => assert!(false),
        }
        let v1 = vec![0x2bu8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        match LocationType::from(&v1) {
            LocationType::PointAlongLine => assert!(true),
            _ => assert!(false),
        }
        let v2 = vec![0x2bu8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        match LocationType::from(&v2) {
            LocationType::Unknown => assert!(true),
            _ => assert!(false),
        }
    }
    #[test]
    fn test_poi() {
        let v0 = vec![
            0x2bu8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        match LocationType::from(&v0) {
            LocationType::PoiWithAccessPoint => assert!(true),
            _ => assert!(false),
        }
        let v1 = vec![
            0x2bu8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        match LocationType::from(&v1) {
            LocationType::PoiWithAccessPoint => assert!(true),
            _ => assert!(false),
        }
        let v2 = vec![
            0x2bu8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        match LocationType::from(&v2) {
            LocationType::Unknown => assert!(true),
            _ => assert!(false),
        }
    }
    #[test]
    fn test_circle() {
        let v0 = vec![0x3u8];
        match LocationType::from(&v0) {
            LocationType::Circle => assert!(true),
            _ => assert!(false),
        }
    }
    #[test]
    fn test_rest() {
        let v0 = vec![0x43u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        match LocationType::from(&v0) {
            LocationType::Rectangle => assert!(true),
            _ => assert!(false),
        }
        let v1 = vec![0x43u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        match LocationType::from(&v1) {
            LocationType::Unknown => assert!(true),
            _ => assert!(false),
        }
        let v2 = vec![0x43u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        match LocationType::from(&v2) {
            LocationType::Rectangle => assert!(true),
            _ => assert!(false),
        }
    }
    #[test]
    fn test_grid() {
        let v0 = vec![0x43u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        match LocationType::from(&v0) {
            LocationType::Grid => assert!(true),
            _ => assert!(false),
        }
        let v1 = vec![0x43u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        match LocationType::from(&v1) {
            LocationType::Unknown => assert!(true),
            _ => assert!(false),
        }
        let v2 = vec![0x43u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        match LocationType::from(&v2) {
            LocationType::Grid => assert!(true),
            _ => assert!(false),
        }
    }
    #[test]
    fn test_poly() {
        let v0 = vec![0x13u8];
        match LocationType::from(&v0) {
            LocationType::Polygon => assert!(true),
            _ => assert!(false),
        }
    }
    #[test]
    fn test_closed_line() {
        let v0 = vec![0x5bu8];
        match LocationType::from(&v0) {
            LocationType::ClosedLine => assert!(true),
            _ => assert!(false),
        }
    }
}
