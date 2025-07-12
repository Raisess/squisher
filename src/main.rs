use squisher::Squisher;

#[derive(serde::Deserialize)]
struct Data {
    pub timestamp: u128,
    pub value: f32,
}

fn main() {
    let file = std::fs::read_to_string("./random_data.json").expect("Failed to open json file");
    let json = serde_json::from_str::<Vec<Data>>(file.as_str())
        .expect("Failed to parse json file content");

    // @NOTE: create a timeseries for the last 24 hours aggregated per 1 hour
    let mut volume_ts = Squisher::new(24, 1f32);
    json.iter()
        .for_each(|item| volume_ts.compute(item.value, item.timestamp));
    println!("{:#?}", volume_ts.ts());

    let volume_24h = volume_ts.ts().iter().fold(0f32, |acc, curr| acc + curr);
    println!("Last 24h volume: {volume_24h}");
}
