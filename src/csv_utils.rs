use std::collections::HashMap;
use csv::StringRecord;

pub fn return_csv_headers() -> Vec<String> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_path("./assets/uspop.csv").unwrap();
    let headers = rdr.headers().unwrap();
    headers.iter().map(|x| x.to_string()).collect()
}

pub fn read_csv_to_rows() -> Result<Vec<StringRecord>, csv::Error> {
    let mut rdr = csv::Reader::from_path("./assets/uspop.csv")?;
    let mut rows = Vec::new();
    for row in rdr.records() {
        let record = row?;
        rows.append(&mut vec![record])
    }
    Ok(rows)
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
