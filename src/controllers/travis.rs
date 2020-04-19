use flate2::write::GzEncoder;
use flate2::Compression;
use tar;

use std::env;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

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
    fn tar_secrets_directory(self) -> Result<(), std::io::Error> {
        let s = "secrets.tar.gz".to_string();
        if Path::new(&s).exists() == true {
            println!("secrets.tar.gz exists");
            fs::remove_file("secrets.tar.gz")?;
            let tar_gz = File::create(&s)?;
            let enc = GzEncoder::new(tar_gz, Compression::default());
            let mut tar = tar::Builder::new(enc);
            tar.append_dir_all("secrets", "secrets")?;
            Ok(())
        } else {
            println!("secrets.tar.gz does not exist");
            let tar_gz = File::create(&s)?;
            let enc = GzEncoder::new(tar_gz, Compression::default());
            let mut tar = tar::Builder::new(enc);
            tar.append_dir_all("secrets", "secrets")?;
            Ok(())
        }
    }
    fn encrypt_tar_secrets(&mut self) {
        let s = "secrets.tar.gz".to_string();
        let output = Command::new("travis")
            .arg("encrypt-file")
            .arg("-f")
            .arg("-p")
            .arg(s)
            .output()
            .expect("travis command failed to start");
        let output_utf = String::from_utf8_lossy(&output.stdout);
        if output.status.success() == true {
            let mut decrypt_cmd = String::new();
            let mut key_var_key = String::new();
            let mut key_var_value = String::new();
            let mut iv_var_key = String::new();
            let mut iv_var_value = String::new();
            for x in output_utf.lines() {
                if x.contains("openssl aes-256-cbc") {
                    decrypt_cmd = x.to_string();
                    for s in decrypt_cmd.split_whitespace() {
                        if s.contains("_key") {
                            key_var_key = s.to_string()
                        } else if s.contains("_iv") {
                            iv_var_key = s.to_string()
                        }
                    }
                } else if x.contains("key:") {
                    key_var_value = x.trim_start_matches("key: ").to_string()
                } else if x.contains("iv:") {
                    iv_var_value = x.trim_start_matches("iv: ").to_string()
                }
            }
            self.decrypt_cmd = decrypt_cmd.to_string();
            self.key_var_key = key_var_key.to_string();
            self.key_var_value = key_var_value.to_string();
            self.iv_var_key = iv_var_key.to_string();
            self.iv_var_value = iv_var_value.to_string();
        }
    }

    fn add_openssl_cmd(self) -> bool {
        let ty = ".travis.yml".to_string();
        let f = fs::read_to_string(".travis.yml");
        let v: serde_json::Value = serde_yaml::from_str(&f.unwrap()).unwrap();
        let mut j = serde_json::json!(&v);
        let bi = j["jobs"]["include"]["before_install"]
            .as_array_mut()
            .unwrap();
        let before_install = v["jobs"]["include"]["before_install"].to_string();
        if before_install.contains(&self.decrypt_cmd) == true {
            println!("decrypt cmd already in .travis.yml")
        } else {
            println!("decrypt_cmd not in .travis.yml");
            bi.push(serde_yaml::from_str(&self.decrypt_cmd.to_string()).unwrap());
        }
        let _ = serde_yaml::to_writer(fs::File::create(ty).unwrap(), &j);
        true
    }
    
    fn decrypt_tar_secrets(self) {
        let u = "secrets.tar.gz".to_string();
        let e = "secrets.tar.gz.enc".to_string();
        let _ = Command::new("openssl")
            .arg("aes-256-cbc")
            .arg("-K")
            .arg(self.key_var_value)
            .arg("-iv")
            .arg(self.iv_var_value)
            .arg("-in")
            .arg(e)
            .arg("-out")
            .arg(u)
            .output()
            .expect("travis command failed to start");
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

pub fn tar_secrets_directory() -> bool {
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
    let _ = c.tar_secrets_directory();
    if Path::new("secrets.tar.gz").exists() == true {
        println!("secrets are tar'd");
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
    let _ = c.encrypt_tar_secrets();
    if Path::new("secrets.tar.gz.enc").exists() == true {
        println!("secrets tar is encrypted");
        true
    } else {
        false
    }
}

pub fn add_openssl_cmd() -> bool {
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
    let result = c.add_openssl_cmd();
    if result == true {
        println!("openssl command added");
        true
    } else {
        false
    }
}
