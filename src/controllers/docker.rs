use std::env;
use std::fs;
use std::io::Error;
use std::io::ErrorKind::NotFound;
use std::path::Path;
use std::process::{Command, Stdio};

pub struct Docker {
    key_var_key: String,
    key_var_value: String,
    iv_var_key: String,
    iv_var_value: String,
}

impl Docker {
    fn build_image(&mut self) -> Result<(), Error> {
        {
            //if secrets exist, set kv
            if Path::new("secrets/travis-openssl-keys-values.txt").exists() == true {
                let f = fs::read_to_string("secrets/travis-openssl-keys-values.txt");
                for s in f.unwrap().lines() {
                    let d: Vec<_> = s.split("=").collect();
                    if d[0].contains("_key") {
                        self.key_var_key = d[0].to_string();
                        self.key_var_value = d[1].to_string();
                    } else if d[0].contains("_iv") {
                        self.iv_var_key = d[0].to_string();
                        self.iv_var_value = d[1].to_string();
                    }
                }
            } else {
                //if does not exist, set k
                let f = fs::read_to_string(".travis-openssl-keys");
                for s in f.unwrap().lines() {
                    let d: Vec<_> = s.split(",").collect();
                    self.key_var_key = d[0].to_string();
                    self.iv_var_key = d[1].to_string();
                }
            }
        }
        {
            //if env k exists, get v
            if env::var_os(&self.key_var_key) != None {
                match env::var_os(&self.key_var_key) {
                    Some(val) => {
                        self.key_var_value = val.into_string().unwrap();
                    }
                    None => println!("{} is not defined in the environment.", &self.key_var_key),
                }
            }
            if env::var_os(&self.iv_var_key) != None {
                match env::var_os(&self.iv_var_key) {
                    Some(val) => {
                        self.iv_var_value = val.into_string().unwrap();
                    }
                    None => println!("{} is not defined in the environment.", &self.iv_var_key),
                }
            }
        }
        {
            match Command::new("docker")
                .stderr(Stdio::null())
                .stdout(Stdio::null())
                .spawn()
            {
                Ok(_) => {
                    //encrypt with set kv
                    println!("docker command found!, building docker image!");
                    let output = Command::new("docker")
                        .args(&[
                            "build",
                            ".",
                            "--build-arg",
                            &format!("key={}", &self.key_var_value),
                            "--build-arg",
                            &format!("iv={}", &self.iv_var_value),
                            "--tag",
                            "securthebox_server_rs:latest",
                        ])
                        .output()
                        .expect("docker command failed to start");
                    println!("building image! takes");
                    if output.status.success() == true {
                        println!("Docker image built");
                    }
                    let s = String::from_utf8_lossy(&output.stdout);
                    println!("{}", s);
                    Ok(())
                }
                Err(e) => {
                    if let NotFound = e.kind() {
                        println!("`docker` was not found! Check your PATH!");
                    } else {
                        println!("Some strange error occurred :(");
                    }
                    Ok(())
                }
            }
        }
    }
}

pub fn build_image() -> bool {
    let mut c = Docker {
        key_var_key: String::new(),
        key_var_value: String::new(),
        iv_var_key: String::new(),
        iv_var_value: String::new(),
    };
    let _ = c.build_image();
    // confirm image is built but needs to be new
    match Command::new("docker")
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .spawn()
    {
        Ok(_) => {
            let output = Command::new("docker")
                .args(&["images", "-q", "securethebox_server_rs:latest"])
                .output()
                .expect("docker command not found");
            let s = String::from_utf8_lossy(&output.stdout);
            println!("Docker Image ID:{}", s);
            if s != "" {
                true
            } else {
                false
            }
        }
        Err(e) => {
            if let NotFound = e.kind() {
                println!("`docker` was not found! Check your PATH!");
                println!("If you see this in travis/docker ignore");
                true
            } else {
                println!("Some strange error occurred :(");
                false
            }
        }
    }
}
