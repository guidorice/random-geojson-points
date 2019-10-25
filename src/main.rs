//! Generate a GeoJSON FeatureCollection with random Point Features.

//! #### Example
//! ```bash
//! cargo install random-geojson-points
//! random-geojson-points > test.geojson
//! // now 100 random points are in test.geojson
//! ```
//! ![map preview](https://github.com/guidorice/random-geojson-points/raw/master/screenshot.png "Look all the random points")
//!
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
