use std::fs::File;

fn main() -> Result<(), csv::Error> {
    let file_path = "data/tips.csv";

    let file = File::open(file_path)?;
    let mut reader = csv::Reader::from_reader(file);

    let headers = reader.headers()?;
    println!("Headers: {headers:?}");

    let mut record_count = 0;

    for result in reader.records() {
        let record = result?;
        record_count += 1;

        if record_count == 1 {
            println!("First record: {record:?}");
        }
    }

    println!("Parsed {record_count} records from {file_path}");

    Ok(())
}
