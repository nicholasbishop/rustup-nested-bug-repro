use std::process::Command;

fn main() {
    let mut cmd = Command::new("cargo");
    cmd.env_remove("RUSTC");
    cmd.env_remove("RUSTDOC");
    cmd.args(&["+nightly", "version"]);

    println!("running command: {:?}", cmd);
    let s = cmd.status().unwrap();
    assert!(s.success());
}
