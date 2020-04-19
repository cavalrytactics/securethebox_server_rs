use std::fs;
use std::process::Command;
// use std::io;
// use std::io::prelude::*;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;

fn main() {
    let output = Command::new("travis")
        .arg("encrypt-file")
        .arg("-f")
        .arg("-p")
        .arg("secrets.tar.gz")
        .output()
        .expect("travis command failed to start");

    let output_utf = String::from_utf8_lossy(&output.stdout);

    if output.status.success() == true {
        let mut decrypt_cmd = String::new();

        for x in output_utf.lines() {
            if x.contains("openssl aes-256-cbc") {
                decrypt_cmd = x.to_string();
            }
        }
        println!("decrypt_cmd: {}", decrypt_cmd);
        add_openssl_cmd(&decrypt_cmd);
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct TravisConfig {
    jobs:           TravisJobsConfig,
    language:       String,
    rust:           Vec<String>
}
#[derive(Serialize, Deserialize, Debug)]
struct TravisJobsConfig {
    include:        TravisIncludeConfig
}
#[derive(Serialize, Deserialize, Debug)]
struct TravisIncludeConfig {
    language:       String,
    before_install: Vec<String>
}

fn add_openssl_cmd(decrypt_cmd: &str){
    let f = fs::read_to_string(".travis.yaml");
    let v: serde_json::Value = serde_yaml::from_str(&f.unwrap()).unwrap();
    let mut j = serde_json::json!(&v);
    let bi = j["jobs"]["include"]["before_install"].as_array_mut().unwrap();
    
    let before_install = v["jobs"]["include"]["before_install"].to_string();
    if before_install.contains(decrypt_cmd) == true {
        println!("decrypt cmd already in .travis.yaml")
    } else {
        println!("decrypt_cmd not in .travis.yaml");
        bi.push(serde_yaml::from_str(&decrypt_cmd.to_string()).unwrap());
    }
    let _ = serde_yaml::to_writer(fs::File::create("test.yaml").unwrap(), &j);

}