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
