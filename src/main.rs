use std::process::Command;
use std::env;

fn main() {
    // Modify the path if any argument is passed in.
    let modify_path = env::args().len() == 2;

    let mut cmd = Command::new("cargo");
    cmd.env_remove("RUSTC");
    cmd.env_remove("RUSTDOC");
    cmd.args(&["+nightly", "version"]);

    let orig_path = env::var_os("PATH").unwrap_or_default();
    println!("Original PATH: {:?}", orig_path);

    if modify_path {
        let modified_split_paths = env::split_paths(&orig_path).filter(|path| {
            !path
                .components()
                .any(|component| component.as_os_str() == ".rustup")
        });
        let modified_path = env::join_paths(modified_split_paths).expect("invalid PATH");
        cmd.env("PATH", &modified_path);

        println!("Modified PATH: {:?}", modified_path);
    }

    println!("running command: {:?}", cmd);
    let s = cmd.status().unwrap();
    assert!(s.success());
}
