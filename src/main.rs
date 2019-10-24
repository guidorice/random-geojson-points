use geojson::{Feature, FeatureCollection, GeoJson, Geometry, Value};
use rand::Rng;
use std::vec::Vec;

const NUM_FEATURES: usize = 100;

// TODO: add commandline options & help banner.

fn main() {
    let mut rng = rand::thread_rng();
    let features: Vec<geojson::Feature> = (0..NUM_FEATURES)
        .map(|_| {
            let longitude = rng.gen_range(-180.0, 180.0);
            let latitude = rng.gen_range(-90.0, 90.0);
            let geometry = Geometry::new(Value::Point(vec![longitude, latitude]));
            Feature {
                bbox: None,
                foreign_members: None,
                geometry: Some(geometry),
                id: None,
                properties: None,
            }
        })
        .collect();

    let geojson = GeoJson::FeatureCollection(FeatureCollection {
        bbox: None,
        features,
        foreign_members: None,
    });
    println!("{}", geojson.to_string());
}
