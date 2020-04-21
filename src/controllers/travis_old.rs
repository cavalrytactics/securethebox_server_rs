use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::io::ErrorKind::NotFound;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use tar::Archive;
use tar::Builder;

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
            fs::remove_file("secrets.tar.gz")?;
            let tar_gz = File::create(&s)?;
            let enc = GzEncoder::new(tar_gz, Compression::default());
            let mut tar = Builder::new(enc);
            tar.append_dir_all("secrets", "secrets")?;
            Ok(())
        } else {
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

        if Path::new(p).exists() == false {
            println!("{} folder does not exist", p.to_string());
            let tar_gz = File::open(s)?;
            let tar = GzDecoder::new(tar_gz);
            let mut archive = Archive::new(tar);
            archive.unpack(".")?;
            println!("unpacked archive!!!");
            Ok(())
        } else {
            println!("{} exists... to not need go decompress", p);
            Ok(())
        }
    }

    fn encrypt_tar_secrets(&mut self) -> Result<(), Error> {
        let s = "secrets.tar.gz";
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
                let f = fs::read_to_string(".travis-openssl-keys");
                for s in f.unwrap().lines() {
                    let d: Vec<_> = s.split(",").collect();
                    self.key_var_key = d[0].to_string();
                    self.iv_var_key = d[1].to_string();
                }
            }
        }
        {
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
        //if travis binary exists
        match Command::new("travis").stdout(Stdio::null()).spawn() {
            Ok(_) => {
                //encrypt with set kv
                let output = Command::new("travis")
                    .arg("encrypt-file")
                    .arg("--key")
                    .arg(&self.key_var_value)
                    .arg("--iv")
                    .arg(&self.iv_var_value)
                    .arg("--force")
                    .arg(s)
                    .output()
                    .expect("travis command failed to start");
                let output_utf = String::from_utf8_lossy(&output.stdout);
                if output.status.success() == true {
                    println!("Encrypted file!");
                    self.decrypt_cmd = format!("openssl aes-256-cbc -K {} -iv {} -in secrets.tar.gz.enc -out secrets.tar.gz -d", &self.key_var_key, &self.iv_var_key);
                    println!("{}",&self.decrypt_cmd);
                }
                Ok(())
            }
            Err(e) => {
                if let NotFound = e.kind() {
                    println!("`travis` was not found! Check your PATH!");
                    println!("If you see this in Travis-CI, ignore");
                } else {
                    println!("Some strange error occurred :(");
                }
                Ok(())
            }
        }
    }
    // fn encrypt_tar_secrets_old(&mut self) -> Result<(), Error> {
    //     let s = "secrets.tar.gz";
    //     // only execute if values not set (we want to prevent values changing)
    //     if Path::new("secrets/travis-openssl-keys-values.txt").exists() == false {
    //         println!("secrets/travis-openssl-keys-values.txt, does not exists");
    //         // if travis binary not installed, install it (except ci)
    //         match Command::new("travis").stdout(Stdio::null()).spawn() {
    //             Ok(_) => {
    //                 let output = Command::new("travis")
    //                     .arg("encrypt-file")
    //                     .arg("-f")
    //                     .arg("-p")
    //                     .arg(s)
    //                     .output()
    //                     .expect("travis command failed to start");
    //                 let output_utf = String::from_utf8_lossy(&output.stdout);
    //                 if output.status.success() == true {
    //                     for x in output_utf.lines() {
    //                         if x.contains("openssl aes-256-cbc") {
    //                             self.decrypt_cmd = x.to_string();
    //                             for s in self.decrypt_cmd.split_whitespace() {
    //                                 if s.contains("_key") {
    //                                     self.key_var_key = s.to_string();
    //                                 } else if s.contains("_iv") {
    //                                     self.iv_var_key = s.to_string()
    //                                 }
    //                             }
    //                         } else if x.contains("key:") {
    //                             self.key_var_value =
    //                                 x.trim_start_matches("key: ").trim_start().to_string();
    //                         } else if x.contains("iv:") {
    //                             self.iv_var_value =
    //                                 x.trim_start_matches("iv: ").trim_start().to_string();
    //                         }
    //                     }
    //                     //thread1
    //                     //write openssl keys to file and env
    //                     {
    //                         let sec_kv_p = "secrets/travis-openssl-keys-values.txt";
    //                         let mut skvf = fs::File::create(sec_kv_p)?;
    //                         let skvs = std::format!(
    //                             "{}={}\n{}={}",
    //                             &self.key_var_key.to_string(),
    //                             &self.key_var_value.to_string(),
    //                             &self.iv_var_key.to_string(),
    //                             &self.iv_var_value.to_string()
    //                         );
    //                         let _ = skvf.write_all(&skvs.as_bytes());
    //                         env::set_var(
    //                             &self.key_var_key.to_string(),
    //                             &self.key_var_value.to_string(),
    //                         );
    //                         env::set_var(
    //                             &self.iv_var_key.to_string(),
    //                             &self.iv_var_value.to_string(),
    //                         );
    //                     }
    //                     //thread2
    //                     //write keys to file for referencing env
    //                     {
    //                         let sec_k_p = ".travis-openssl-keys";
    //                         let mut skf = fs::File::create(sec_k_p)?;
    //                         let sks = std::format!(
    //                             "{},{}",
    //                             &self.key_var_key.to_string(),
    //                             &self.iv_var_key.to_string(),
    //                         );
    //                         let _ = skf.write_all(&sks.as_bytes());
    //                     }
    //                 }
    //             }
    //             Err(e) => {
    //                 if let NotFound = e.kind() {
    //                     println!("`travis` was not found! Check your PATH!");
    //                     println!("If you see this in Travis-CI, ignore");
    //                 } else {
    //                     println!("Some strange error occurred :(");
    //                 }
    //             }
    //         }
    //         Ok(())
    //     } else {
    //         println!("secrets/travis-openssl-keys-values.txt, exists...");
    //         // Encrypt using existing variableso
    //         // get decrypt values from file (used for apps)
    //         {
    //             if Path::new("secrets/travis-openssl-keys-values.txt").exists() == true {
    //                 let f = fs::read_to_string("secrets/travis-openssl-keys-values.txt");
    //                 for s in f.unwrap().lines() {
    //                     let d: Vec<_> = s.split("=").collect();
    //                     if d[0].contains("_key") {
    //                         self.key_var_key = d[0].to_string();
    //                         self.key_var_value = d[1].to_string();
    //                     } else if d[0].contains("_iv") {
    //                         self.iv_var_key = d[0].to_string();
    //                         self.iv_var_value = d[1].to_string();
    //                     }
    //                 }
    //             }
    //         }
    //         Ok(())
    //     }
    // }
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
        let s = "secrets.tar.gz.enc";
        let o = "secrets.tar.gz";
        let output = Command::new("travis")
            .arg("encrypt-file")
            .arg("-d")
            .arg("--key")
            .arg(&self.key_var_value)
            .arg("--iv")
            .arg(&self.iv_var_value)
            .arg("--force")
            .arg(s)
            .output()
            .expect("openssl command failed to start");
        let output_utf = String::from_utf8_lossy(&output.stdout);
        if output.status.success() == true {
            println!("Unencrypted file!")
        }
    }
    // fn decrypt_tar_secrets_old(&mut self) {
    //     {
    //         if Path::new("secrets.tar.gz").exists() == true {
    //             let _ = fs::remove_file("secrets.tar.gz");
    //             println!("deleted secrets.tar.gz");
    //         }
    //     }
    //     // get decrypt values from injected env (used for ci)
    //     {
    //         // get keys from file
    //         let f = fs::read_to_string(".travis-openssl-keys");
    //         for s in f.unwrap().lines() {
    //             let d: Vec<_> = s.split(",").collect();
    //             self.key_var_key = d[0].to_string();
    //             self.iv_var_key = d[1].to_string();
    //         }
    //         // get values from env
    //         match env::var_os(&self.key_var_key) {
    //             Some(val) => {
    //                 self.key_var_value = val.into_string().unwrap();
    //             }
    //             None => println!("{} is not defined in the environment.", &self.key_var_key),
    //         }
    //         match env::var_os(&self.iv_var_key) {
    //             Some(val) => {
    //                 self.iv_var_value = val.into_string().unwrap();
    //             }
    //             None => println!("{} is not defined in the environment.", &self.iv_var_key),
    //         }
    //     }

    //     // get decrypt values from file (used for apps)
    //     {
    //         if Path::new("secrets/travis-openssl-keys-values.txt").exists() == true {
    //             let f = fs::read_to_string("secrets/travis-openssl-keys-values.txt");
    //             for s in f.unwrap().lines() {
    //                 let d: Vec<_> = s.split("=").collect();
    //                 if d[0].contains("_key") {
    //                     self.key_var_key = d[0].to_string();
    //                     self.key_var_value = d[1].to_string();
    //                 } else if d[0].contains("_iv") {
    //                     self.iv_var_key = d[0].to_string();
    //                     self.iv_var_value = d[1].to_string();
    //                 }
    //             }
    //         }
    //     }

    //     let e = "secrets.tar.gz.enc";
    //     let u = "secrets.tar.gz";
    //     let _ = Command::new("openssl")
    //         .arg("aes-256-cbc")
    //         .arg("-K")
    //         .arg(&self.key_var_value)
    //         .arg("-iv")
    //         .arg(&self.iv_var_value)
    //         .arg("-in")
    //         .arg(e)
    //         .arg("-out")
    //         .arg(u)
    //         .arg("-d")
    //         .output()
    //         .expect("travis command failed to start");
    //     if Path::new("secrets.tar.gz").exists() == true {
    //         println!("secrets.tar.gz exists")
    //     }
    // }
}
//public functions should be STATELESS
//add all the necessary sub functions
//when used for tests, all public functions are ran in parallel
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
    let _ = c.tar_compress_secrets_directory();
    let _ = c.tar_decompress_secrets_directory();
    if Path::new("secrets/travis-openssl-keys-values.txt").exists() == true {
        println!("tar decompressed");
        true
    } else {
        println!("error");
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
    // c.add_openssl_cmd();
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
    let _ = c.encrypt_tar_secrets();
    c.add_openssl_cmd();
    c.decrypt_tar_secrets();
    let _ = c.tar_decompress_secrets_directory();
    if Path::new("secrets/travis-openssl-keys-values.txt").exists() == true {
        println!("secrets tar is decrypted");
        true
    } else {
        false
    }
}
