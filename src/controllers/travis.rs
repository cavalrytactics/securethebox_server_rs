use flate2::write::GzEncoder;
use flate2::write::GzDecoder;
use flate2::Compression;
use tar::Builder;
use tar::Archive;

use std::env;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::io::Error;
use std::io::ErrorKind::NotFound;
use std::io::prelude::*;

pub struct Travis {
    current_directory: PathBuf,
    file_name: String,
    decrypt_cmd: String,
    key_var_key: String,
    key_var_value: String,
    iv_var_key: String,
    iv_var_value: String,
}

impl Travis {
    fn set_file_name(&mut self, file_name: &str) {
        self.file_name = file_name.to_string();
    }
    fn set_current_directory(&mut self) {
        self.current_directory = env::current_dir().unwrap();
    }
    fn tar_compress_secrets_directory(&mut self) -> Result<(), Error> {
        let s = "secrets.tar.gz".to_string();
        if Path::new(&s).exists() == true {
            println!("secrets.tar.gz exists");
            fs::remove_file("secrets.tar.gz")?;
            let tar_gz = File::create(&s)?;
            let enc = GzEncoder::new(tar_gz, Compression::default());
            let mut tar = Builder::new(enc);
            tar.append_dir_all("secrets", "secrets")?;
            Ok(())
        } else {
            println!("secrets.tar.gz does not exist");
            let tar_gz = File::create(&s)?;
            let enc = GzEncoder::new(tar_gz, Compression::default());
            let mut tar = Builder::new(enc);
            tar.append_dir_all("secrets", "secrets")?;
            Ok(())
        }
    }
    fn tar_decompress_secrets_directory(&mut self) -> Result<(), Error> {
        let s = "secrets.tar.gz";
        let p = "secrets";
        if Path::new(s).exists() == true {
            let tar_gz = File::open(s)?;  
            let tar = GzDecoder::new(tar_gz);
            let mut archive = Archive::new(tar);
            if Path::new(p).exists() == true {
                println!("decompressed secrets already exists!");
                println!("you are running this locally");
            } else {
                archive.unpack(".")?;
                println!("secrets decompressed");
            }
            Ok(())
        } else {
            println!("secrets.tar.gz does not exist");
            Ok(())
        }
    }
    fn encrypt_tar_secrets(&mut self) -> Result<(), Error>{
        let s = "secrets.tar.gz";
        match Command::new("travis").stdout(Stdio::null()).spawn(){
            Ok(_) => {
                let output = Command::new("travis")
                    .arg("encrypt-file")
                    .arg("-f")
                    .arg("-p")
                    .arg(s)
                    .output()
                    .expect("travis command failed to start");
                let output_utf = String::from_utf8_lossy(&output.stdout);
                if output.status.success() == true {
                    for x in output_utf.lines() {
                        if x.contains("openssl aes-256-cbc") {
                            self.decrypt_cmd = x.to_string();
                            for s in self.decrypt_cmd.split_whitespace() {
                                if s.contains("_key") {
                                    self.key_var_key = s.to_string();
                                } else if s.contains("_iv") {
                                    self.iv_var_key = s.to_string()
                                }
                            }
                            
                        } else if x.contains("key:") {
                            self.key_var_value = x.trim_start_matches("key: ")
                                                  .trim_start()
                                                  .to_string();
                        } else if x.contains("iv:") {
                            self.iv_var_value = x.trim_start_matches("iv: ")
                                                 .trim_start()
                                                 .to_string();
                        }
                    }
                    //thread1
                    {
                        let sec_kv_p = "secrets/travis-openssl-keys-values.txt";
                        let mut skvf = fs::File::create(sec_kv_p)?;
                        let mut skvs = std::format!("{}={}\n{}={}",
                            &self.key_var_key.to_string(), 
                            &self.key_var_value.to_string(),
                            &self.iv_var_key.to_string(), 
                            &self.iv_var_value.to_string()
                        );
                        skvf.write_all(&skvs.as_bytes());
                    }
                    //thread2
                    {
                        let sec_k_p = "secrets/travis-openssl-keys";
                        let mut skf = fs::File::create(sec_k_p)?;
                        let mut sks = std::format!("{},{}",
                            &self.key_var_key.to_string(), 
                            &self.iv_var_key.to_string(), 
                        );
                        skf.write_all(&sks.as_bytes());
                    }
                }
            },
            Err(e) => {
                if let NotFound = e.kind() {
                    println!("`travis` was not found! Check your PATH!");
                    println!("If you see this in Travis-CI, ignore");
                } else {
                    println!("Some strange error occurred :(");
                }
            }, 
        }
        Ok(())
    }
    
    fn add_openssl_cmd(&mut self) {
        let ty = ".travis.yml".to_string();
        let f = fs::read_to_string(".travis.yml");
        let v: serde_json::Value = serde_yaml::from_str(&f.unwrap()).unwrap();
        let mut j = serde_json::json!(&v);
        let bi = j["jobs"]["include"]["before_install"]
            .as_array_mut()
            .unwrap();
        let before_install = v["jobs"]["include"]["before_install"].to_string();
        if before_install.contains(&self.decrypt_cmd) == true {
            println!("decrypt_cmd already in .travis.yml")
        } else {
            println!("decrypt_cmd not in .travis.yml, adding...");
            bi.push(serde_yaml::from_str(&self.decrypt_cmd).unwrap());
        }
        let _ = serde_yaml::to_writer(fs::File::create(ty).unwrap(), &j);
    }
    
    fn decrypt_tar_secrets(&mut self) {
        if Path::new("secrets.tar.gz").exists() == true {
            let _ = fs::remove_file("secrets.tar.gz");
            println!("deleted secrets.tar.gz")
        }

        let e = "secrets.tar.gz.enc";
        let u = "secrets.tar.gz";
        let _ = Command::new("openssl")
            .arg("aes-256-cbc")
            .arg("-K")
            .arg(&self.key_var_value)
            .arg("-iv")
            .arg(&self.iv_var_value)
            .arg("-in")
            .arg(e)
            .arg("-out")
            .arg(u)
            .arg("-d")
            .output()
            .expect("travis command failed to start");
        if Path::new("secrets.tar.gz").exists() == true {
            println!("secrets.tar.gz exists")
        }
    }
}

pub fn set_file_name(arg_file_name: &str) -> bool {
    let mut c = Travis {
        current_directory: PathBuf::new(),
        file_name: String::new(),
        decrypt_cmd: String::new(),
        key_var_key: String::new(),
        key_var_value: String::new(),
        iv_var_key: String::new(),
        iv_var_value: String::new(),
    };
    c.set_file_name(arg_file_name);
    if c.file_name == arg_file_name {
        println!("file name: {}", c.file_name);
        true
    } else {
        false
    }
}

pub fn set_current_directory() -> bool {
    let mut c = Travis {
        current_directory: PathBuf::new(),
        file_name: String::new(),
        decrypt_cmd: String::new(),
        key_var_key: String::new(),
        key_var_value: String::new(),
        iv_var_key: String::new(),
        iv_var_value: String::new(),
    };
    c.set_current_directory();
    if c.current_directory != PathBuf::new() {
        println!("current directory: {:?}", c.current_directory);
        true
    } else {
        false
    }
}

pub fn tar_compress_secrets_directory() -> bool {
    let mut c = Travis {
        current_directory: PathBuf::new(),
        file_name: String::new(),
        decrypt_cmd: String::new(),
        key_var_key: String::new(),
        key_var_value: String::new(),
        iv_var_key: String::new(),
        iv_var_value: String::new(),
    };
    c.set_current_directory();
    let _ = c.tar_compress_secrets_directory();
    if Path::new("secrets.tar.gz").exists() == true {
        println!("tar compressed");
        true
    } else {
        false
    }
}

pub fn tar_decompress_secrets_directory() -> bool {
    let mut c = Travis {
        current_directory: PathBuf::new(),
        file_name: String::new(),
        decrypt_cmd: String::new(),
        key_var_key: String::new(),
        key_var_value: String::new(),
        iv_var_key: String::new(),
        iv_var_value: String::new(),
    };
    c.set_current_directory();
    let _ = c.tar_decompress_secrets_directory();
    if Path::new("secrets/secret.txt").exists() == true {
        println!("tar decompressed");
        true
    } else {
        false
    }
}

pub fn encrypt_tar_secrets() -> bool {
    let mut c = Travis {
        current_directory: PathBuf::new(),
        file_name: String::new(),
        decrypt_cmd: String::new(),
        key_var_key: String::new(),
        key_var_value: String::new(),
        iv_var_key: String::new(),
        iv_var_value: String::new(),
    };
    c.set_current_directory();
    c.encrypt_tar_secrets();
    c.add_openssl_cmd();
    if Path::new("secrets.tar.gz.enc").exists() == true {
        println!("secrets tar is encrypted");
        true
    } else {
        false
    }
}

pub fn decrypt_tar_secrets() -> bool {
    let mut c = Travis {
        current_directory: PathBuf::new(),
        file_name: String::new(),
        decrypt_cmd: String::new(),
        key_var_key: String::new(),
        key_var_value: String::new(),
        iv_var_key: String::new(),
        iv_var_value: String::new(),
    };
    c.set_current_directory();
    let _ = c.tar_compress_secrets_directory();
    c.encrypt_tar_secrets();
    c.add_openssl_cmd();
    c.decrypt_tar_secrets();
    let _ = c.tar_decompress_secrets_directory();
    if Path::new("secrets.tar.gz").exists() == true {
        println!("secrets tar is decrypted");
        true
    } else {
        false
    }
}