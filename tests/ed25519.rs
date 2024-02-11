use assert_cmd::Command;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn correct_sign() -> TestResult {
    let mut cmd = Command::cargo_bin("sign")?;

    cmd.args([
        "tests/samples/key",
        "tests/samples/message",
        "tmp_signature_sign",
    ])
    .assert()
    .success();
    let expected_signature = fs::read("tests/samples/signature")?;
    let signature = fs::read("tmp_signature_sign")?;
    assert_eq!(signature, expected_signature);
    fs::remove_file("tmp_signature_sign")?;
    Ok(())
}

#[test]
fn correct_verify() -> TestResult {
    let mut cmd = Command::cargo_bin("verify")?;

    cmd.args([
        "tests/samples/key.pk",
        "tests/samples/message",
        "tests/samples/signature",
    ])
    .assert()
    .success()
    .stdout("ACCEPT\n");
    Ok(())
}

#[test]
fn tampered_verify_1() -> TestResult {
    let mut cmd = Command::cargo_bin("verify")?;

    cmd.args([
        "tests/samples/key.pk",
        "tests/samples/tampered_message",
        "tests/samples/signature",
    ])
    .assert()
    .success()
    .stdout("REJECT\n");
    Ok(())
}

#[test]
fn tampered_verify_2() -> TestResult {
    let mut cmd = Command::cargo_bin("verify")?;

    cmd.args([
        "tests/samples/key.pk",
        "tests/samples/message",
        "tests/samples/tampered_signature",
    ])
    .assert()
    .success()
    .stdout("REJECT\n");
    Ok(())
}

#[test]
fn correct_flow() -> TestResult {
    let mut keygen = Command::cargo_bin("keygen")?;
    let mut sign = Command::cargo_bin("sign")?;
    let mut verify = Command::cargo_bin("verify")?;

    keygen.args(["tmp_key"]).assert().success();

    sign.args(["tmp_key", "tests/samples/message", "tmp_signature_flow"])
        .assert()
        .success();

    verify
        .args(["tmp_key.pk", "tests/samples/message", "tmp_signature_flow"])
        .assert()
        .success()
        .stdout("ACCEPT\n");

    fs::remove_file("tmp_key.sk")?;
    fs::remove_file("tmp_key.pk")?;
    fs::remove_file("tmp_signature_flow")?;

    Ok(())
}
