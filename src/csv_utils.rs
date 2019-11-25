use std::collections::HashMap;

fn return_csv_headers() -> Vec<String> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_path("./assets/uspop.csv").unwrap();
    let headers = rdr.headers().unwrap();
    headers.iter().map(|x| x.to_string()).collect()
}

pub fn read_csv_to_rows() -> Result<HashMap<String, Vec<String>>, csv::Error> {
    let mut rdr = csv::Reader::from_path("./assets/uspop.csv")?;
    let hashmap = rdr
        .deserialize()
        .filter_map(|val| val.ok())
        .enumerate()
        .map(|(index, val)| (format!("row {}", index), val))
        .collect::<HashMap<String, Vec<String>>>();
    Ok(hashmap)
}

pub fn read_csv_to_columns() -> Result<HashMap<String, Vec<String>>, csv::Error>{
    let mut rdr = csv::Reader::from_path("./assets/uspop.csv")?;
    let mut columns = HashMap::new();
    for result in rdr.deserialize() {
        let record: HashMap<String, String> = result?;
        for (k, v) in record.iter() {
            columns.entry(String::from(k)).or_insert(Vec::new()).push(String::from(v))
        }
    }

    Ok(columns)

}
