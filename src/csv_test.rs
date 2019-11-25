
fn read_csv_to_rows() -> Result<HashMap<String, Vec<String>>, csv::Error> {
    let mut rdr = csv::Reader::from_path("./assets/uspop.csv")?;
    let hashmap = rdr
        .deserialize()
        .filter_map(|val| val.ok())
        .enumerate()
        .map(|(index, val)| (format!("row {}", index), val))
        .collect::<HashMap<String, Vec<String>>>();
    // .enumerate()
    // .map(|(index, val)| (format!("column {}", index % 5), val))
    Ok(hashmap)
}

fn main() {
    let a = read_csv_to_rows().unwrap();
    for record in a {
        println!("{:?}", record)
    }
}
