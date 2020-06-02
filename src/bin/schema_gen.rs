use std::fs;
use std::process::Command;

fn main() {
    let output = Command::new("graphql-client")
        .args(&["introspect-schema", "http://127.0.0.1:8000"])
        .output()
        .expect("graphql-client command failed to start");
    let _ = fs::write("schema.json", &output.stdout);
}
