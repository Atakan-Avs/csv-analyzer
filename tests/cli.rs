use std::process::Command;

#[test]
fn summarizes_tips_csv() {
    let output = Command::new(env!("CARGO_BIN_EXE_csv-analyzer"))
        .arg("data/tips.csv")
        .output()
        .expect("failed to run csv-analyzer");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("expected UTF-8 output");
    assert!(stdout.contains("Parsed 244 records"));
    assert!(stdout.contains("Average tip: 3.00"));
    assert!(stdout.contains("Highest tip: 10.00"));
}

#[test]
fn summarizes_tips_csv_for_selected_day() {
    let output = Command::new(env!("CARGO_BIN_EXE_csv-analyzer"))
        .args(["data/tips.csv", "Sun"])
        .output()
        .expect("failed to run csv-analyzer");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("expected UTF-8 output");
    assert!(stdout.contains("Parsed 76 records"));
    assert!(stdout.contains("Total bill: 1627.16"));
    assert!(stdout.contains("Total tips: 247.39"));
    assert!(stdout.contains("Average tip: 3.26"));
    assert!(stdout.contains("Highest tip: 6.50"));
}

#[test]
fn reports_missing_csv_file() {
    let output = Command::new(env!("CARGO_BIN_EXE_csv-analyzer"))
        .arg("data/__missing_file_for_check__.csv")
        .output()
        .expect("failed to run csv-analyzer");

    assert_eq!(output.status.code(), Some(1));

    let stderr = String::from_utf8(output.stderr).expect("expected UTF-8 output");
    assert!(stderr.contains("Failed to open CSV file 'data/__missing_file_for_check__.csv':"));
}

#[test]
fn reports_no_records_for_unmatched_day() {
    let output = Command::new(env!("CARGO_BIN_EXE_csv-analyzer"))
        .args(["data/tips.csv", "Mon"])
        .output()
        .expect("failed to run csv-analyzer");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("expected UTF-8 output");
    assert!(stdout.contains("Parsed 0 records"));
    assert!(stdout.contains("No records to analyze."));
    assert!(!stdout.contains("Total bill:"));
    assert!(!stdout.contains("Total tips:"));
    assert!(!stdout.contains("Average tip:"));
    assert!(!stdout.contains("Highest tip:"));
}

#[test]
fn shows_help() {
    let output = Command::new(env!("CARGO_BIN_EXE_csv-analyzer"))
        .arg("--help")
        .output()
        .expect("failed to run csv-analyzer");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("expected UTF-8 output");
    assert!(stdout.contains("Usage: csv-analyzer <file_path> [day]"));
    assert!(stdout.contains("Example: csv-analyzer data/tips.csv Sun"));
    assert!(output.stderr.is_empty());
}

#[test]
fn rejects_extra_arguments() {
    let output = Command::new(env!("CARGO_BIN_EXE_csv-analyzer"))
        .args(["data/tips.csv", "Sun", "extra"])
        .output()
        .expect("failed to run csv-analyzer");

    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());

    let stderr = String::from_utf8(output.stderr).expect("expected UTF-8 output");
    assert!(stderr.contains("Too many arguments."));
    assert!(stderr.contains("Usage: csv-analyzer <file_path> [day]"));
}

#[test]
fn rejects_invalid_tip_value() {
    let output = Command::new(env!("CARGO_BIN_EXE_csv-analyzer"))
        .arg("tests/fixtures/invalid_tip.csv")
        .output()
        .expect("failed to run csv-analyzer");

    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8(output.stderr).expect("expected UTF-8 output");
    assert!(
        stderr.starts_with("Failed to read a CSV record from 'tests/fixtures/invalid_tip.csv':")
    );

    let stdout = String::from_utf8(output.stdout).expect("expected UTF-8 output");
    assert!(stdout.contains("First record:"));
    assert!(!stdout.contains("Total bill:"));
    assert!(!stdout.contains("Total tips:"));
    assert!(!stdout.contains("Average tip:"));
    assert!(!stdout.contains("Highest tip:"));
}

#[test]
fn reports_added_removed_and_changed_csv_rows() {
    let output = Command::new(env!("CARGO_BIN_EXE_csv-analyzer"))
        .args([
            "diff",
            "tests/fixtures/diff_before.csv",
            "tests/fixtures/diff_after.csv",
        ])
        .output()
        .expect("failed to run csv-analyzer");

    assert!(output.status.success());
    assert!(output.stderr.is_empty());

    let stdout = String::from_utf8(output.stdout).expect("expected UTF-8 output");
    assert_eq!(
        stdout.lines().collect::<Vec<_>>(),
        vec![
            "Added: id=102",
            "Removed: id=101",
            "Changed: id=100, empty_weight: 12000 -> 12450",
        ]
    );
}

#[test]
fn rejects_different_csv_column_order() {
    let output = Command::new(env!("CARGO_BIN_EXE_csv-analyzer"))
        .args([
            "diff",
            "tests/fixtures/diff_before.csv",
            "tests/fixtures/diff_reordered_columns.csv",
        ])
        .output()
        .expect("failed to run csv-analyzer");

    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());

    let stderr = String::from_utf8(output.stderr).expect("expected UTF-8 output");
    assert_eq!(
        stderr.trim(),
        "CSV files must have the same columns in the same order."
    );
}

#[test]
fn matches_csv_rows_by_selected_column() {
    let output = Command::new(env!("CARGO_BIN_EXE_csv-analyzer"))
        .args([
            "diff",
            "tests/fixtures/diff_by_plate_before.csv",
            "tests/fixtures/diff_by_plate_after.csv",
            "--key",
            "plate",
        ])
        .output()
        .expect("failed to run csv-analyzer");

    assert!(output.status.success());
    assert!(output.stderr.is_empty());

    let stdout = String::from_utf8(output.stdout).expect("expected UTF-8 output");
    assert_eq!(
        stdout.trim_end(),
        "Changed: plate=TEST-001, empty_weight: 12000 -> 12450"
    );
}
