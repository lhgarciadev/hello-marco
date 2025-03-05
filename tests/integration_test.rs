use assert_cmd::Command;

#[test]
fn test_marco_polo() {
    let mut cmd = Command::cargo_bin("hello-marco").unwrap();
    cmd.arg("play").arg("--name").arg("Marco");
    cmd.assert().success().stdout("Polo\n");
}

#[test]
fn test_marco_polo_not_marco() {
    let mut cmd = Command::cargo_bin("hello-marco").unwrap();
    cmd.arg("play").arg("--name").arg("John");
    cmd.assert().success().stdout("What?\n");
}
