use serde::Deserialize;
use std::fs::File;

#[derive(Deserialize)]
struct TipRecord {
    total_bill: f64,
    tip: f64,
    sex: String,
    smoker: String,
    day: String,
    time: String,
    size: u8,
}

fn main() -> Result<(), csv::Error> {
    let file_path = "data/tips.csv";

    let file = File::open(file_path)?;
    let mut reader = csv::Reader::from_reader(file);

    let headers = reader.headers()?;
    println!("Headers: {headers:?}");

    let mut record_count = 0;

    for result in reader.deserialize::<TipRecord>() {
        let record = result?;
        record_count += 1;

        if record_count == 1 {
            println!(
                "First record: total_bill={}, tip={}, sex={}, smoker={}, day={}, time={}, size={}",
                record.total_bill,
                record.tip,
                record.sex,
                record.smoker,
                record.day,
                record.time,
                record.size,
            );
        }
    }

    println!("Parsed {record_count} records from {file_path}");

    Ok(())
}
