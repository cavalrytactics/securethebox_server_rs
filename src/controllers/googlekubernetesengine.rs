use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Error;
use std::io::ErrorKind::NotFound;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub struct GoogleKubernetesEngine {
    file_name: String,
    current_directory: PathBuf,
    service_name: String,
    user_name: String,
    email_address: String,
    kubectl_action: String,
    kubernetes_deployment_image: String,
    kubernetes_deployment_name: String,
    google_project_id: String,
    google_kubernetes_compute_zone: String,
    google_kubernetes_compute_cluster: String,
    google_kubernetes_compute_region: String,
    google_service_account_email: String,
    kubernetes_pod_id: String,
    environment_variables: HashMap<String, String>,
}

impl GoogleKubernetesEngine {
    fn set_file_name(&mut self, file_name: &str) {
        self.file_name = file_name.to_string();
    }
    fn set_current_directory(&mut self) {
        self.current_directory = env::current_dir().unwrap();
    }
    fn set_service_name(&mut self, service_name: &str) {
        self.service_name = service_name.to_string();
    }
    fn set_user_name(&mut self, user_name: &str) {
        self.user_name = user_name.to_string();
    }
    fn set_email_address(&mut self, email_address: &str) {
        self.email_address = email_address.to_string();
    }
    fn set_environment_variable(&mut self, environment_variable: &str) {
        let v = environment_variable.to_string();
        if env::var_os(environment_variable) != None {
            match env::var_os(environment_variable) {
                Some(val) => {
                    self.environment_variables
                        .entry(v)
                        .or_insert(val.into_string().unwrap());
                }
                None => println!("{} is not defined in the environment.",v),
            }
        } else {
            println!("{} is not defined in the environment.",v);
        }
    }
    fn set_google_kubernetes_deployment_name(&mut self, kubernetes_deployment_name: &str) {
        self.kubernetes_deployment_name = kubernetes_deployment_name.to_string();
    }
    fn set_google_kubernetes_deployment_image(&mut self, kubernetes_deployment_image: &str) {
        self.kubernetes_deployment_image = kubernetes_deployment_image.to_string();
    }
    fn set_kubernetes_pod_id(&mut self, kubernetes_pod_id: &str) {
        self.kubernetes_pod_id = kubernetes_pod_id.to_string();
    }
    fn set_kubectl_action(&mut self, kubectl_action: &str) {
        self.kubectl_action = kubectl_action.to_string();
    }
    fn set_google_project_id(&mut self, google_project_id: &str) {
        self.google_project_id = google_project_id.to_string();
        let output = Command::new("gcloud")
            .args(&["config","set", "project",&self.google_project_id])
            .output()
            .expect("gcloud command failed to start");
        if output.status.success() == true {
            println!("set google project id!")
        }
    }
    fn set_google_kubernetes_compute_zone(&mut self, google_kubernetes_compute_zone: &str) {
        self.google_kubernetes_compute_zone = google_kubernetes_compute_zone.to_string();
    }
    fn set_google_kubernetes_compute_cluster(&mut self, google_kubernetes_compute_cluster: &str) {
        self.google_kubernetes_compute_cluster = google_kubernetes_compute_cluster.to_string();
    }
    fn set_google_kubernetes_compute_region(&mut self, google_kubernetes_compute_region: &str) {
        self.google_kubernetes_compute_region = google_kubernetes_compute_region.to_string();
    }
    fn set_google_service_account_email(&mut self, google_service_account_email: &str) {
        self.google_service_account_email = google_service_account_email.to_string()
    }
    fn load_google_kubernetes_service_account(self) {
        let output = Command::new("gcloud")
            .args(&[ "auth", "activate-service-account", "--key-file",
            &format!("{}/secrets/{}",String::from_utf8_lossy(
                &self.current_directory.to_str().unwrap().as_bytes()), 
                &self.file_name)])
            .output()
            .expect("gcloud command failed to start");
        if output.status.success() == true {
            println!("loaded service account file!")
        }
        let output2 = Command::new("gcloud")
            .args(&[ "config", "set", "account", 
                &self.google_service_account_email])
            .output()
            .expect("gcloud command failed to start");
        if output2.status.success() == true {
            println!("set service account!")
        }
    }
    fn create_google_kubernetes_cluster(self) {
        let output = Command::new("gcloud")
            .args(&[ "auth", "activate-service-account", "--key-file",
            &format!("{}/secrets/{}", String::from_utf8_lossy( 
                &self.current_directory.to_str().unwrap().as_bytes()), 
                &self.file_name)])
            .output()
            .expect("gcloud command failed to start");
        if output.status.success() == true {
            println!("loaded service account file!")
        }
        let output2 = Command::new("gcloud")
            .args(&[ "config", "set", "account", 
                &self.google_service_account_email])
            .output()
            .expect("gcloud command failed to start");
        if output2.status.success() == true {
            println!("set service account!")
        }
        let output3 = Command::new("gcloud")
            // Limit on .args is 32. need to split into multiple
            .args(&["container", 
                "--project", &self.google_project_id, "clusters", "create", 
                    &self.google_kubernetes_compute_cluster, 
                "--zone", &self.google_kubernetes_compute_zone,
                "--no-enable-basic-auth",
                "--release-channel", "stable",
                "--machine-type", "n1-standard-1",
                "--image-type", "COS",
                "--disk-type", "pd-standard",
                "--disk-size", "100"])
            .args(&["--scopes", 
                    "https://www.googleapis.com/auth/devstorage.read_only",
                    "https://www.googleapis.com/auth/logging.write",
                    "https://www.googleapis.com/auth/monitoring",
                    "https://www.googleapis.com/auth/servicecontrol",
                    "https://www.googleapis.com/auth/service.management.readonly",
                    "https://www.googleapis.com/auth/ndev.clouddns.readwrite",
                    "https://www.googleapis.com/auth/trace.append",
                "--num-nodes", "4",
                "--enable-ip-alias",
                "--enable-stackdriver-kubernetes",
                "--addons",
                    "HorizontalPodAutoscaling",
                    "HttpLoadBalancing",
                    "CloudRun",
                "--network",
                    &format!("projects/{}/global/networks/default", 
                        &self.google_project_id),
                "--subnetwork",
                    &format!("projects/{}/regions/{}/subnetworks/default", 
                        &self.google_project_id, 
                        &self.google_kubernetes_compute_region),
                "--default-max-pods-per-node", "8" ])
            .output()
            .expect("gcloud command failed to start");
        if output3.status.success() == true {
            println!("set service account!")
        }
    }
    fn get_google_kubernetes_cluster_credentials(self){
        let output1 = Command::new("gcloud")
            .args(&["auth", "activate-service-account", "--key-file", 
                &format!("{}/secrets/{}",
                String::from_utf8_lossy(
                    &self.current_directory.to_str().unwrap().as_bytes()), 
                    &self.file_name)])
            .output()
            .expect("gcloud command failed to start");
        if output1.status.success() == true {
            println!("logged in with file!")
        }
        let output2 = Command::new("gcloud")
            .args(&[ "auth", "activate-service-account", "--key-file", 
                &format!("{}/secrets/{}",String::from_utf8_lossy(
                    &self.current_directory.to_str().unwrap().as_bytes()), 
                    &self.file_name)])
            .output()
            .expect("gcloud command failed to start");
        if output2.status.success() == true {
            println!("set service account!")
        }
        let output3 = Command::new("gcloud")
            .args(&[ "container", "clusters", "get-credentials", 
                &self.google_kubernetes_compute_cluster, 
                "--project", &self.google_project_id,
                "--zone", &self.google_kubernetes_compute_zone
                ])
            .output()
            .expect("gcloud command failed to start");
        if output3.status.success() == true {
            println!("got creds!")
        }
    }

    fn create_cluster_role_binding(self) {
        let output1 = Command::new("kubectl")
            .args(&[ "create", "clusterrolebinding", "external-dns", 
                "--clusterrole=cluster-admin", 
                "--user=stb-kubernetes-engine-sa@securethebox-server.iam.gserviceaccount.com"
             ])
            .output()
            .expect("gcloud command failed to start");
        if output1.status.success() == true {
            println!("set service account!")
        }
    }
    fn delete_cluster_role_binding(self) {
        let output1 = Command::new("kubectl")
            .args(&["create", "clusterrolebinding", "external-dns", 
                "--clusterrole=cluster-admin", 
                "--user=stb-kubernetes-engine-sa@securethebox-server.iam.gserviceaccount.com"
             ])
            .output()
            .expect("gcloud command failed to start");
        if output1.status.success() == true {
            println!("set service account!")
        }
    }
    fn delete_google_kubernetes_cluster(self) {
        let output = Command::new("gcloud")
            .args(&[ "auth", "activate-service-account", "--key-file",
            &format!("{}/secrets/{}", String::from_utf8_lossy( 
                &self.current_directory.to_str().unwrap().as_bytes()), 
                &self.file_name)])
            .output()
            .expect("gcloud command failed to start");
        if output.status.success() == true {
            println!("loaded service account file!")
        }
        let output2 = Command::new("gcloud")
            .args(&[ "config", "set", "account", 
                &self.google_service_account_email])
            .output()
            .expect("gcloud command failed to start");
        if output2.status.success() == true {
            println!("set service account!")
        }
        let output2 = Command::new("gcloud")
            .args(&[ "container", "clusters", "delete", 
                &self.google_kubernetes_compute_cluster,
                "--project", &self.google_project_id,
                "--zone", &self.google_kubernetes_compute_zone,
                "--quiet"
                ])
            .output()
            .expect("gcloud command failed to start");
        if output2.status.success() == true {
            println!("set service account!")
        }
    }
    fn helper_delete_orphan_object(self, object_id: &str, object_type: &str){

    }

    //TODO: fn generate_yaml_ingress_files
    //TODO: fn generate_yaml_service_files
    //TODO: fn generate_yaml_authentication_files
    //TODO: fn generate_yaml_storage_files
    //TODO: fn generate_yaml_dns_files
    //TODO: fn delete_yaml_ingress_files
    //TODO: fn delete_yaml_service_files
    //TODO: fn delete_yaml_authentication_files
    //TODO: fn delete_yaml_storage_files
    //TODO: fn delete_yaml_dns_files


}
