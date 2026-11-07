# CSV Analyzer

A small Rust command-line tool that reads a CSV file of restaurant bills and tips. It reports the record count, total bill amount, total tips, and average tip.

## Usage

```bash
cargo run -- data/tips.csv
```

The sample file comes from Seaborn. To analyze another file, pass its path in place of `data/tips.csv`. The file must include these columns:

```text
total_bill,tip,sex,smoker,day,time,size
```
