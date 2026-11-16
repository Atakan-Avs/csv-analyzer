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
    let Some(file_path) = std::env::args().nth(1) else {
        eprintln!("Usage: csv-analyzer <file_path> [day]");
        std::process::exit(1);
    };

    let day_filter = std::env::args().nth(2);

    let file = match File::open(&file_path) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Failed to open CSV file '{file_path}': {error}");
            std::process::exit(1);
        }
    };
    let mut reader = csv::Reader::from_reader(file);

    let headers = reader.headers()?;
    println!("Headers: {headers:?}");

    let mut record_count: u32 = 0;
    let mut total_bill = 0.0;
    let mut total_tip = 0.0;
    let mut max_tip = 0.0;

    for result in reader.deserialize::<TipRecord>() {
        let record = result?;
        if let Some(day) = &day_filter
            && record.day != *day
        {
            continue;
        }
        record_count += 1;
        total_bill += record.total_bill;
        total_tip += record.tip;
        if record_count == 1 || record.tip > max_tip {
            max_tip = record.tip;
        }

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

    if record_count == 0 {
        println!("No records to analyze.");
        return Ok(());
    }

    println!("Total bill: {total_bill:.2}");
    println!("Total tips: {total_tip:.2}");

    let average_tip = total_tip / f64::from(record_count);
    println!("Average tip: {average_tip:.2}");
    println!("Highest tip: {max_tip:.2}");

    Ok(())
}
