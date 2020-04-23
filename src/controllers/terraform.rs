https://learn.hashicorp.com/terraform/development/running-terraform-in-automation


terraform init -input=false
terraform apply -input=false -auto-approve && sleep 60 && terraform destroy -input=false -auto-approve

terraform workspace select USERNAME
pub struct Terraform {
    google_cloud_keyfile_json_key: String,
    google_cloud_keyfile_json_value: String,
}

impl Terraform {
    fn set_google_cloud_keyfile_json_path(self) {
        if env::var_os(&self.google_cloud_keyfile_json_key) != None {
            match env::var_os(&self.google_cloud_keyfile_json_key) {
                Some(val) => {
                    self.google_cloud_keyfile_json_value = val.into_string().unwrap();
                }
                None => println!("{} is not defined in the environment.", &self.google_cloud_keyfile_json_key),
            }
        }
    }
    fn init(self){
        let output = Command::new("terraform")
            .args(&["init", "-input=false"])
            .output()
            .expect("terraform command failed to start");
        if output.status.success() == true {
            println!("Terraform init complete!");
        }
    }
    fn plan(self){
        let output = Command::new("terraform")
            .args(&["plan", "-input=false", "-out=tfplan"])
            .output()
            .expect("terraform command failed to start");
        if output.status.success() == true {
            println!("Terraform plan complete!");
        }
    }
    fn apply(self){
        let output = Command::new("terraform")
            .args(&["apply", "-input=false", "-auto-approve"])
            .output()
            .expect("terraform command failed to start");
        if output.status.success() == true {
            println!("Terraform apply complete!");
        }
    }
    fn destroy(self){
        let output = Command::new("terraform")
            .args(&["destroy", "-input=false", "-auto-approve"])
            .output()
            .expect("terraform command failed to start");
        if output.status.success() == true {
            println!("Terraform destroy complete!");
        }
    }
}

