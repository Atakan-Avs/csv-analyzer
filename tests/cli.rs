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
}
