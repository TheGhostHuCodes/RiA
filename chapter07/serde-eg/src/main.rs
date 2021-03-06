use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct City {
    name: String,
    population: u64,
    latitude: f64,
    longitude: f64,
}

fn main() {
    let calabar = City {
        name: "Calibar".to_string(),
        population: 470_000,
        latitude: 4.95,
        longitude: 8.33,
    };

    let as_json = serde_json::to_string(&calabar).unwrap();
    let as_cbor = serde_cbor::to_vec(&calabar).unwrap();
    let as_bincode = bincode::serialize(&calabar).unwrap();

    println!("json: {}", &as_json);
    println!("cbor: {:?}", &as_cbor);
    println!("cbor (as UTF-8): {:?}", String::from_utf8_lossy(&as_cbor));
    println!("bincode: {:?}", &as_bincode);
    println!(
        "bincode (as UTF-8): {:?}",
        String::from_utf8_lossy(&as_bincode)
    );
}
