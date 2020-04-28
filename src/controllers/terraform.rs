// https://learn.hashicorp.com/terraform/development/running-terraform-in-automation

use std::env;
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

pub struct Terraform {
    google_service: String,
    google_cloud_keyfile: String,
    terraform_path: String,
    terraform_workspace_name: String,
    kubernetes_cluster_name: String,
}

impl Terraform {
    // Allow changing key to use different service accounts
    fn set_google_service(&mut self, google_service: &String) {
        self.google_service = google_service.to_string()
    }
    fn set_terraform_path(&mut self, terraform_path: &String) {
        self.terraform_path = terraform_path.to_string()
    }
    fn set_terraform_workspace_name(&mut self, terraform_workspace_name: &String) {
        self.terraform_workspace_name = terraform_workspace_name.to_string()
    }
    fn set_kubernetes_cluster_name(&mut self, kubernetes_cluster_name: &String) {
        self.kubernetes_cluster_name = kubernetes_cluster_name.to_string()
    }
    fn set_google_cloud_keyfile(&mut self) {
        let s = String::from(&self.google_service);
        match s.as_str() {
            "cloudrun" => {
                env::set_var(
                    "GOOGLE_CLOUD_KEYFILE_JSON",
                    "./secrets/securethebox-client-stb-cloud-run-sa.json",
                );
                self.google_cloud_keyfile =
                    "./secrets/securethebox-client-stb-cloud-run-sa.json".to_string();
            }
            "cloudtask" => {
                env::set_var(
                    "GOOGLE_CLOUD_KEYFILE_JSON",
                    "./secrets/securethebox-client-stb-cloud-task-sa.json",
                );
                self.google_cloud_keyfile =
                    "./secrets/securethebox-client-stb-cloud-task-sa.json".to_string();
            }
            "kubernetes" => {
                env::set_var(
                    "GOOGLE_CLOUD_KEYFILE_JSON",
                    "./secrets/securethebox-server-stb-kubernetes-engine-sa.json",
                );
                self.google_cloud_keyfile =
                    "./secrets/securethebox-server-stb-kubernetes-engine-sa.json".to_string();
            }
            _ => {
                env::set_var(
                    "GOOGLE_CLOUD_KEYFILE_JSON",
                    "./secrets/securethebox-server-stb-kubernetes-engine-sa.json",
                );
                self.google_cloud_keyfile =
                    "./secrets/securethebox-server-stb-kubernetes-engine-sa.json".to_string();
            }
        }
    }
    fn remove_terraform_cache(&mut self) {
        let _ = fs::remove_dir_all(".terraform/");
    }
    fn create_terraform_workspace(&mut self, terraform_workspace_name: &String) {
        self.terraform_workspace_name = terraform_workspace_name.to_string();
        let output = Command::new("terraform")
            .args(&["workspace", "new", terraform_workspace_name])
            .output()
            .expect("terraform command failed to start");
        if output.status.success() == true {
            println!("Terraform workspace {} created!", terraform_workspace_name);
        }
    }
    fn select_terraform_workspace(&mut self, terraform_workspace_name: &String) {
        let output = Command::new("terraform")
            .args(&["workspace", "select", terraform_workspace_name])
            .output()
            .expect("terraform command failed to start");
        if output.status.success() == true {
            println!("Terraform workspace {} selected!", terraform_workspace_name);
        }
    }
    // cannot delete a workspace that is currenty selected
    fn delete_terraform_workspace(&mut self, terraform_workspace_name: &String) {
        self.select_terraform_workspace(&"main".to_string());
        let output = Command::new("terraform")
            .args(&["workspace", "delete", terraform_workspace_name])
            .output()
            .expect("terraform command failed to start");
        let s = String::from_utf8_lossy(&output.stderr);
        println!("delete_terraform_workspace OUTPUT:{:?}{:?}",output.stdout, s);    
        if output.status.success() == true {
            println!("Terraform workspace {} deleted!", terraform_workspace_name);
        }
    }
    fn init(&mut self) {
        let output = Command::new("terraform")
            .args(&["init", "-input=false", &self.terraform_path])
            .output()
            .expect("terraform command failed to start");
        if output.status.success() == true {
            // let s = String::from_utf8_lossy(&output.stdout);
            println!("Terraform init complete!");
        }
    }
    fn plan(&mut self) {
        // do not use "-out", may contain secrets 
        // https://www.terraform.io/docs/commands/plan.html#security-warning    
        let mut output = Command::new("terraform")
            .args(&[
                "plan",
                "-input=false",
                "-var",
                &format!("kubernetes_cluster_name={}", &self.kubernetes_cluster_name),
                &format!(
                    "-out=terraform.tfstate.d/{}/tfplan",
                    &self.terraform_workspace_name
                ),
                &self.terraform_path,
            ])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .unwrap();
        let status = output.wait();
        println!("Exited with status {:?}", status);
    }
    fn apply(&mut self) {
        let mut output = Command::new("terraform")
            .args(&[
                "apply",
                "-input=false",
                "-var",
                &format!("kubernetes_cluster_name={}", &self.kubernetes_cluster_name),
                "-auto-approve",
                &self.terraform_path,
            ])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .unwrap();
        let status = output.wait();
        println!("Exited with status {:?}", status);
    }
    fn destroy(&mut self) {
        let mut output = Command::new("terraform")
            .args(&[
                "destroy",
                "-input=false",
                "-var",
                &format!("kubernetes_cluster_name={}", &self.kubernetes_cluster_name),
                "-auto-approve",
                &self.terraform_path,
            ])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .unwrap();
        let status = output.wait();
        println!("Exited with status {:?}", status);
    }
}

pub fn set_google_service(arg_google_service: String) -> bool {
    let mut c = Terraform {
        google_service: String::new(),
        google_cloud_keyfile: String::new(),
        terraform_path: String::new(),
        terraform_workspace_name: String::new(),
        kubernetes_cluster_name: String::new(),
    };
    c.set_google_service(&arg_google_service);
    if c.google_service == arg_google_service {
        true
    } else {
        false
    }
}

pub fn set_terraform_path(arg_terraform_path: String) -> bool {
    let mut c = Terraform {
        google_service: String::new(),
        google_cloud_keyfile: String::new(),
        terraform_path: String::new(),
        terraform_workspace_name: String::new(),
        kubernetes_cluster_name: String::new(),
    };
    c.set_terraform_path(&arg_terraform_path);
    if c.terraform_path == arg_terraform_path {
        true
    } else {
        false
    }
}

pub fn set_terraform_workspace_name(arg_terraform_workspace_name: String) -> bool {
    let mut c = Terraform {
        google_service: String::new(),
        google_cloud_keyfile: String::new(),
        terraform_path: String::new(),
        terraform_workspace_name: String::new(),
        kubernetes_cluster_name: String::new(),
    };
    c.set_terraform_workspace_name(&arg_terraform_workspace_name);
    if c.terraform_workspace_name == arg_terraform_workspace_name {
        true
    } else {
        false
    }
}
pub fn set_kubernetes_cluster_name(arg_kubernetes_cluster_name: String) -> bool {
    let mut c = Terraform {
        google_service: String::new(),
        google_cloud_keyfile: String::new(),
        terraform_path: String::new(),
        terraform_workspace_name: String::new(),
        kubernetes_cluster_name: String::new(),
    };
    c.set_kubernetes_cluster_name(&arg_kubernetes_cluster_name);
    if c.kubernetes_cluster_name == arg_kubernetes_cluster_name {
        true
    } else {
        false
    }
}
pub fn set_google_cloud_keyfile(arg_google_service: String) -> bool {
    let mut c = Terraform {
        google_service: String::new(),
        google_cloud_keyfile: String::new(),
        terraform_path: String::new(),
        terraform_workspace_name: String::new(),
        kubernetes_cluster_name: String::new(),
    };
    c.set_google_service(&arg_google_service);
    c.set_google_cloud_keyfile();
    if c.google_service != "" {
        true
    } else {
        false
    }
}

pub fn remove_terraform_cache() -> bool {
    let mut c = Terraform {
        google_service: String::new(),
        google_cloud_keyfile: String::new(),
        terraform_path: String::new(),
        terraform_workspace_name: String::new(),
        kubernetes_cluster_name: String::new(),
    };
    c.remove_terraform_cache();
    // going to make another attempt (for parallel tests)
    if Path::new(".terraform").exists() == false {
        true
    } else {
        false
    }
}
pub fn create_terraform_workspace(arg_terraform_workspace_name: String) -> bool {
    let mut c = Terraform {
        google_service: String::new(),
        google_cloud_keyfile: String::new(),
        terraform_path: String::new(),
        terraform_workspace_name: String::new(),
        kubernetes_cluster_name: String::new(),
    };
    c.set_terraform_workspace_name(&arg_terraform_workspace_name);
    c.create_terraform_workspace(&"main".to_string());
    c.create_terraform_workspace(&arg_terraform_workspace_name);
    if Path::new(&format!(
        "./terraform.tfstate.d/{}",
        &arg_terraform_workspace_name
    ))
    .exists()
        == true
    {
        true
    } else {
        false
    }
}

pub fn select_terraform_workspace(arg_terraform_workspace_name: String) -> bool {
    let mut c = Terraform {
        google_service: String::new(),
        google_cloud_keyfile: String::new(),
        terraform_path: String::new(),
        terraform_workspace_name: String::new(),
        kubernetes_cluster_name: String::new(),
    };
    c.set_terraform_workspace_name(&arg_terraform_workspace_name);
    c.create_terraform_workspace(&"main".to_string());
    c.create_terraform_workspace(&arg_terraform_workspace_name);
    c.select_terraform_workspace(&arg_terraform_workspace_name);
    if Path::new(&format!(
        "./terraform.tfstate.d/{}",
        &arg_terraform_workspace_name
    ))
    .exists()
        == true
    {
        true
    } else {
        false
    }
}

pub fn delete_terraform_workspace(arg_terraform_workspace_name: String) -> bool {
    let mut c = Terraform {
        google_service: String::new(),
        google_cloud_keyfile: String::new(),
        terraform_path: String::new(),
        terraform_workspace_name: String::new(),
        kubernetes_cluster_name: String::new(),
    };
    c.set_terraform_workspace_name(&arg_terraform_workspace_name);
    c.create_terraform_workspace(&"main".to_string());
    c.create_terraform_workspace(&arg_terraform_workspace_name);
    c.select_terraform_workspace(&"main".to_string());
    c.delete_terraform_workspace(&arg_terraform_workspace_name);
    if Path::new(&format!(
        "./terraform.tfstate.d/{}",
        &arg_terraform_workspace_name
    ))
    .exists()
        == false
    {
        true
    } else {
        false
    }
}

pub fn init(arg_google_service: String, arg_terraform_path: String) -> bool {
    let mut c = Terraform {
        google_service: String::new(),
        google_cloud_keyfile: String::new(),
        terraform_path: String::new(),
        terraform_workspace_name: String::new(),
        kubernetes_cluster_name: String::new(),
    };
    c.set_google_service(&arg_google_service);
    c.set_terraform_path(&arg_terraform_path);
    c.set_google_cloud_keyfile();
    c.remove_terraform_cache();
    c.init();
    if Path::new(".terraform").exists() == true {
        true
    } else {
        false
    }
}

pub fn plan(
    arg_google_service: String,
    arg_terraform_path: String,
    arg_terraform_workspace_name: String,
    arg_kubernetes_cluster_name: String,
) -> bool {
    let mut c = Terraform {
        google_service: String::new(),
        google_cloud_keyfile: String::new(),
        terraform_path: String::new(),
        terraform_workspace_name: String::new(),
        kubernetes_cluster_name: String::new(),
    };
    c.set_google_service(&arg_google_service);
    c.set_terraform_path(&arg_terraform_path);
    c.set_google_cloud_keyfile();
    c.init();
    c.set_terraform_workspace_name(&arg_terraform_workspace_name);
    c.set_kubernetes_cluster_name(&arg_kubernetes_cluster_name);
    c.create_terraform_workspace(&arg_terraform_workspace_name);
    c.select_terraform_workspace(&arg_terraform_workspace_name);
    c.plan();
    if Path::new(&format!(
        "terraform.tfstate.d/{}/tfplan",
        &arg_terraform_workspace_name
    ))
    .exists()
        == true
    {
        true
    } else {
        false
    }
}

pub fn apply(
    arg_google_service: String,
    arg_terraform_path: String,
    arg_terraform_workspace_name: String,
    arg_kubernetes_cluster_name: String,
) -> bool {
    let mut c = Terraform {
        google_service: String::new(),
        google_cloud_keyfile: String::new(),
        terraform_path: String::new(),
        terraform_workspace_name: String::new(),
        kubernetes_cluster_name: String::new(),
    };
    c.set_google_service(&arg_google_service);
    c.set_terraform_path(&arg_terraform_path);
    c.set_google_cloud_keyfile();
    c.init();
    c.set_terraform_workspace_name(&arg_terraform_workspace_name);
    c.set_kubernetes_cluster_name(&arg_kubernetes_cluster_name);
    c.create_terraform_workspace(&arg_terraform_workspace_name);
    c.select_terraform_workspace(&arg_terraform_workspace_name);
    c.apply();
    if Path::new(&format!(
        "terraform.tfstate.d/{}/tfplan",
        &arg_terraform_workspace_name
    ))
    .exists()
        == true
    {
        true
    } else {
        false
    }
}

pub fn destroy(
    arg_google_service: String,
    arg_terraform_path: String,
    arg_terraform_workspace_name: String,
    arg_kubernetes_cluster_name: String,
) -> bool {
    let mut c = Terraform {
        google_service: String::new(),
        google_cloud_keyfile: String::new(),
        terraform_path: String::new(),
        terraform_workspace_name: String::new(),
        kubernetes_cluster_name: String::new(),
    };
    c.set_google_service(&arg_google_service);
    c.set_terraform_path(&arg_terraform_path);
    c.set_google_cloud_keyfile();
    c.init();
    c.set_terraform_workspace_name(&arg_terraform_workspace_name);
    c.set_kubernetes_cluster_name(&arg_kubernetes_cluster_name);
    c.create_terraform_workspace(&arg_terraform_workspace_name);
    c.select_terraform_workspace(&arg_terraform_workspace_name);
    c.destroy();
    if Path::new(&format!(
        "terraform.tfstate.d/{}/tfplan",
        &arg_terraform_workspace_name
    ))
    .exists()
        == true
    {
        true
    } else {
        false
    }
}
