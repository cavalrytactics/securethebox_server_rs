use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Error;
use std::io::ErrorKind::NotFound;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use tar::Archive;
use tar::Builder;
use yaml_rust::{YamlEmitter, YamlLoader};

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
    fn set_file_name(&mut self, file_name: &String) {
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
            println!("{} exists... do not need go decompress", p);
            Ok(())
        }
    }

    fn encrypt_tar_secrets(&mut self) -> Result<(), Error> {
        let s = "secrets.tar.gz";
        // create a scope
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
        } // <-- borrow ends here
          // create a scope
        {
            //it env k exists, get v
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
        } // <-- borrow ends here
          //if travis binary exists, start encrypt
        match Command::new("travis").stdout(Stdio::null()).spawn() {
            Ok(_) => {
                //encrypt with set kv
                let _ = Command::new("travis")
                    .args(&[
                        "encrypt-file",
                        "--key",
                        &self.key_var_value,
                        "--iv",
                        &self.iv_var_value,
                        "--force",
                        s,
                    ])
                    .output()
                    .expect("travis command failed to start");
                if &self.key_var_key != "" && &self.iv_var_key != "" {
                    self.decrypt_cmd = format!(
                        "openssl aes-256-cbc -K {} -iv {} -in secrets.tar.gz.enc -out secrets.tar.gz -d  && tar xvf secrets.tar.gz", 
                        &self.key_var_key, &self.iv_var_key
                    );
                }
                Ok(())
            }
            Err(e) => {
                if let NotFound = e.kind() {
                    println!("`travis` was not found! Check your PATH!");
                    println!("If you see this in Travis-CI, safe to ignore");
                } else {
                    println!("Some strange error occurred :(");
                }
                Ok(())
            }
        }
    }
    //add decrypt command to .tarvis.yml
    fn add_openssl_cmd(&mut self) {
        let f = fs::read_to_string(".travis_template.yml");
        let v: serde_json::Value = serde_yaml::from_str(&f.unwrap()).unwrap();
        let mut j = serde_json::json!(&v);
        let bi = j["jobs"]["include"]["before_install"]
            .as_array_mut()
            .unwrap();
        let before_install = v["jobs"]["include"]["before_install"].to_string();
        if before_install.contains(&self.decrypt_cmd) == true {
        } else {
            bi.push(serde_yaml::from_str(&self.decrypt_cmd).unwrap());
        }
        {
            let s = serde_yaml::to_string(&j);
            let _ = fs::write(".travis.yml", &snailquote::unescape(&s.unwrap()).unwrap());
        }
    }

    fn decrypt_tar_secrets(&mut self) {
        let s = "secrets.tar.gz.enc";
        let o = "secrets.tar.gz";
        match Command::new("travis").stdout(Stdio::null()).spawn() {
            Ok(_) => {
                let output = Command::new("travis")
                    .args(&[
                        "encrypt-file",
                        "-d",
                        "--key",
                        &self.key_var_value,
                        "--iv",
                        &self.iv_var_value,
                        "--force",
                        s,
                    ])
                    .output()
                    .expect("openssl command failed to start");
                if output.status.success() == true {
                    println!("Unencrypted file!")
                }
            }
            Err(e) => {
                if let NotFound = e.kind() {
                    println!("`travis` was not found! Check your PATH!");
                    println!("If you see this in Travis-CI, safe to ignore");
                    let f = fs::read_to_string(".travis-openssl-keys");
                    for s in f.unwrap().lines() {
                        let d: Vec<_> = s.split(",").collect();
                        self.key_var_key = d[0].to_string();
                        self.iv_var_key = d[1].to_string();
                    }
                    if env::var_os(&self.key_var_key) != None {
                        match env::var_os(&self.key_var_key) {
                            Some(val) => {
                                self.key_var_value = val.into_string().unwrap();
                            }
                            None => {
                                println!("{} is not defined in the environment.", &self.key_var_key)
                            }
                        }
                    }
                    if env::var_os(&self.iv_var_key) != None {
                        match env::var_os(&self.iv_var_key) {
                            Some(val) => {
                                self.iv_var_value = val.into_string().unwrap();
                            }
                            None => {
                                println!("{} is not defined in the environment.", &self.iv_var_key)
                            }
                        }
                    }
                    let output = Command::new("openssl")
                        .args(&[
                            "aes-256-cbc",
                            "-K",
                            &self.key_var_value,
                            "-iv",
                            &self.iv_var_value,
                            "-in",
                            s,
                            "-out",
                            o,
                            "-d",
                        ])
                        .output()
                        .expect("openssl command failed to start");
                    if output.status.success() == true {
                        println!("Unencrypted file!")
                    }
                } else {
                    println!("Some strange error occurred :(");
                }
            }
        }
    }
}
// public functions should be STATELESS
// add all the necessary sub functions
// when used for tests, all public functions are by default ran in parallel
// https://doc.rust-lang.org/book/ch11-02-running-tests.html#running-tests-in-parallel-or-consecutively
pub fn set_file_name(arg_file_name: String) -> bool {
    let mut c = Travis {
        current_directory: PathBuf::new(),
        file_name: String::new(),
        decrypt_cmd: String::new(),
        key_var_key: String::new(),
        key_var_value: String::new(),
        iv_var_key: String::new(),
        iv_var_value: String::new(),
    };
    c.set_file_name(&arg_file_name);
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
    match Command::new("travis").stdout(Stdio::null()).spawn() {
        Ok(_) => {
            if Path::new("secrets/travis-openssl-keys-values.txt").exists() == true {
                println!("tar decompressed");
                true
            } else {
                println!("error: tar_decompress_secrets_directory");
                false
            }
        }
        Err(e) => {
            if let NotFound = e.kind() {
                println!("If you see this in Travis-CI, safe to ignore");
                true
            } else {
                println!("Some strange error occurred :(");
                false
            }
        }
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
    let _ = c.encrypt_tar_secrets();
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
    let _ = c.encrypt_tar_secrets();
    c.add_openssl_cmd();
    c.decrypt_tar_secrets();
    let _ = c.tar_decompress_secrets_directory();
    match Command::new("travis").stdout(Stdio::null()).spawn() {
        Ok(_) => {
            if Path::new("secrets.tar.gz").exists() == true {
                println!("secrets tar is decrypted and decompressed");
                true
            } else {
                println!("error, Ok decrypt_tar_secrets");
                false
            }
        }
        Err(e) => {
            if let NotFound = e.kind() {
                println!("If you see this in Travis-CI, safe to ignore");
                if Path::new("secrets.tar.gz").exists() == true {
                    println!("secrets tar is decrypted and decompressed");
                    true
                } else {
                    println!("error, Err decrypt_tar_secrets");
                    false
                }
            } else {
                println!("Some strange error occurred :(");
                false
            }
        }
    }
}
