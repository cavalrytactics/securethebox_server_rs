// use std::collections::HashMap;
// use std::env;
// use std::fs;
// use std::fs::File;
// use std::io::Error;
// use std::io::ErrorKind::NotFound;
// use std::path::{Path, PathBuf};
// use std::process::{Command, Stdio};

// pub struct GoogleKubernetesEngine {
//     file_name: String,
//     current_directory: PathBuf,
//     service_name: String,
//     user_name: String,
//     email_address: String,
//     kubectl_action: String,
//     kubernetes_deployment_image: String,
//     kubernetes_deployment_name: String,
//     google_project_id: String,
//     google_kubernetes_compute_zone: String,
//     google_kubernetes_compute_cluster: String,
//     google_kubernetes_compute_region: String,
//     google_service_account_email: String,
//     kubernetes_pod_id: String,
//     environment_variables: HashMap<String, String>,
// }

// impl GoogleKubernetesEngine {
//     fn set_file_name(&mut self, file_name: &String) {
//         self.file_name = file_name.to_string();
//     }
//     fn set_current_directory(&mut self) {
//         self.current_directory = env::current_dir().unwrap();
//     }
//     fn set_service_name(&mut self, service_name: &String) {
//         self.service_name = service_name.to_string();
//     }
//     fn set_user_name(&mut self, user_name: &String) {
//         self.user_name = user_name.to_string();
//     }
//     fn set_email_address(&mut self, email_address: &String) {
//         self.email_address = email_address.to_string();
//     }
//     fn set_environment_variable(&mut self, environment_variable: &String) {
//         let v = environment_variable.to_string();
//         if env::var_os(environment_variable) != None {
//             match env::var_os(environment_variable) {
//                 Some(val) => {
//                     self.environment_variables
//                         .entry(v)
//                         .or_insert(val.into_string().unwrap());
//                 }
//                 None => println!("{} is not defined in the environment.",v),
//             }
//         } else {
//             println!("{} is not defined in the environment.",v);
//         }
//     }
//     fn set_google_kubernetes_deployment_name(&mut self, kubernetes_deployment_name: &String) {
//         self.kubernetes_deployment_name = kubernetes_deployment_name.to_string();
//     }
//     fn set_google_kubernetes_deployment_image(&mut self, kubernetes_deployment_image: &String) {
//         self.kubernetes_deployment_image = kubernetes_deployment_image.to_string();
//     }
//     fn set_google_project_id(&mut self, google_project_id: &String) {
//         self.google_project_id = google_project_id.to_string();
//         let output = Command::new("gcloud")
//             .args(&["config","set", "project",&self.google_project_id])
//             .output()
//             .expect("gcloud command failed to start");
//         if output.status.success() == true {
//             println!("set google project id!")
//         }
//     }
//     fn set_google_kubernetes_compute_zone(&mut self, google_kubernetes_compute_zone: &String) {
//         self.google_kubernetes_compute_zone = google_kubernetes_compute_zone.to_string();
//     }
//     fn set_google_kubernetes_compute_cluster(&mut self, google_kubernetes_compute_cluster: &String) {
//         self.google_kubernetes_compute_cluster = google_kubernetes_compute_cluster.to_string();
//     }
//     fn set_google_kubernetes_compute_region(&mut self, google_kubernetes_compute_region: &String) {
//         self.google_kubernetes_compute_region = google_kubernetes_compute_region.to_string();
//     }
//     fn set_google_service_account_email(&mut self, google_service_account_email: &String) {
//         self.google_service_account_email = google_service_account_email.to_string()
//     }
//     fn load_google_kubernetes_service_account(self) {
//         let output = Command::new("gcloud")
//             .args(&[ "auth", "activate-service-account", "--key-file",
//             &format!("{}/secrets/{}",String::from_utf8_lossy(
//                 &self.current_directory.to_str().unwrap().as_bytes()), 
//                 &self.file_name)])
//             .output()
//             .expect("gcloud command failed to start");
//         if output.status.success() == true {
//             println!("loaded service account file!")
//         }
//         let output2 = Command::new("gcloud")
//             .args(&[ "config", "set", "account", 
//                 &self.google_service_account_email])
//             .output()
//             .expect("gcloud command failed to start");
//         if output2.status.success() == true {
//             println!("set service account!")
//         }
//     }

//     fn get_google_kubernetes_cluster_credentials(self){
//         let output1 = Command::new("gcloud")
//             .args(&["auth", "activate-service-account", "--key-file", 
//                 &format!("{}/secrets/{}",
//                 String::from_utf8_lossy(
//                     &self.current_directory.to_str().unwrap().as_bytes()), 
//                     &self.file_name)])
//             .output()
//             .expect("gcloud command failed to start");
//         if output1.status.success() == true {
//             println!("logged in with file!")
//         }
//         let output2 = Command::new("gcloud")
//             .args(&[ "auth", "activate-service-account", "--key-file", 
//                 &format!("{}/secrets/{}",String::from_utf8_lossy(
//                     &self.current_directory.to_str().unwrap().as_bytes()), 
//                     &self.file_name)])
//             .output()
//             .expect("gcloud command failed to start");
//         if output2.status.success() == true {
//             println!("set service account!")
//         }
//         let output3 = Command::new("gcloud")
//             .args(&[ "container", "clusters", "get-credentials", 
//                 &self.google_kubernetes_compute_cluster, 
//                 "--project", &self.google_project_id,
//                 "--zone", &self.google_kubernetes_compute_zone
//                 ])
//             .output()
//             .expect("gcloud command failed to start");
//         if output3.status.success() == true {
//             println!("got creds!")
//         }
//     }

//     // fn helper_delete_orphan_object(self, object_id: &String, object_type: &String){


//     // }
// pub fn set_file_name_GoogleKubernetesEngine(arg_file_name: String) -> bool {
//     let mut c = GoogleKubernetesEngine {
//         file_name: String::new(),
//         current_directory: PathBuf::new(),
//         service_name: String::new(),
//         user_name: String::new(),
//         email_address: String::new(),
//         kubectl_action: String::new(),
//         kubernetes_deployment_image: String::new(),
//         kubernetes_deployment_name: String::new(),
//         google_project_id: String::new(),
//         google_kubernetes_compute_zone: String::new(),
//         google_kubernetes_compute_cluster: String::new(),
//         google_kubernetes_compute_region: String::new(),
//         google_service_account_email: String::new(),
//         kubernetes_pod_id: String::new(),
//         environment_variables: HashMap::new(),
//     };
//     c.set_file_name(&arg_file_name);
//     if c.file_name == arg_file_name {
//         println!("file name: {}", c.file_name);
//         true
//     } else {
//         false
//     }
// }
// // def helper_deleteOrphanObject(self, objectId: str, objectType: str) -> bool:
// //     print("Deleting:", objectId, objectType)
// //     if objectType == "firewall-rules":
// //         subprocess.Popen([f"gcloud compute --project=\"{self.googleProjectId}\" -q firewall-rules delete {objectId}"],shell=True).wait()
// //     elif objectType == "target-pools":
// //         subprocess.Popen([f"gcloud compute --project=\"{self.googleProjectId}\" -q target-pools delete {objectId} --region={self.googleKubernetesComputeRegion}"],shell=True).wait()
// //     elif objectType == "backend-services":
// //         subprocess.Popen([f"gcloud compute --project=\"{self.googleProjectId}\" -q backend-services delete {objectId} --region={self.googleKubernetesComputeRegion}"],shell=True).wait()
// //     elif objectType == "forwarding-rules":
// //         subprocess.Popen([f"gcloud compute --project=\"{self.googleProjectId}\" -q forwarding-rules delete {objectId} --region={self.googleKubernetesComputeRegion}"],shell=True).wait()
// //     elif objectType == "health-checks":
// //         subprocess.Popen([f"gcloud compute --project=\"{self.googleProjectId}\" -q health-checks delete {objectId}"],shell=True).wait()
// //     elif objectType == "addresses":
// //         subprocess.Popen([f"echo 'y' | gcloud compute --project=\"{self.googleProjectId}\" addresses delete {objectId}"],shell=True).wait()

// // def helper_checkValidFirewallRule(self, objectId: str) -> bool:
// //     command = ["gcloud",f"--project=\"{self.googleProjectId}\"","compute","firewall-rules","describe",f"\"{objectId}\"","--format=json"]
// //     out = check_output(command)
// //     fw_json=json.loads(out)
// //     description=fw_json["description"] # $(jq -r .description <<<"$fw_json")
// //     service_name=fw_json["kubernetes.io/service-name"] # $(jq -r '."kubernetes.io/service-name"' <<<"$description")
// //     ip=fw_json["kubernetes.io/service-ip"] # $(jq -r '."kubernetes.io/service-ip"' <<<"$description")
// //     print(f"=> {objectId}, IP: {ip}, Service: {service_name}")
// //     if ip not in self.activeIps:
// //         # IP not in use
// //         return False
// //     else:
// //         # IP is in use
// //         return True
    
// // def deleteFirewallRules(self) -> bool:
// //     try:
// //         command = ["gcloud",f"--project={self.googleProjectId}","compute","firewall-rules","list","--format=value(name)",f"--filter=name ~ ^k8s"]
// //         out = check_output(command)
// //         object_list = out.decode('utf-8')
// //         object_split_list = object_list.splitlines()
// //         for objectId in object_split_list:
// //             self.helper_deleteOrphanObject(objectId, "firewall-rules")
// //         print(u"\n\n \u001b[48;5;196" + "m " +"IF YOU SEE ERRORS HERE, YOU NEED TO DELETE MANUALLY!\u001b[0m")
// //         print(f"\nhttps://console.cloud.google.com/networking/firewalls/list?project={self.googleProjectId}&authuser=2&addressesTablesize=50\n")
// //         return True
// //     except:
// //         return False

// // def deleteFirewallRulesTest(self) -> bool:
// //     try:
// //         command = ["gcloud",f"--project={self.googleProjectId}","compute","firewall-rules","list","--format=value(name)",f"--filter=name ~ ^k8s"]
// //         out = check_output(command)
// //         object_list = out.decode('utf-8')
// //         object_split_list = object_list.splitlines()
// //         for objectId in object_split_list:
// //             print(objectId)
// //             self.helper_checkValidFirewallRule(objectId)
// //         return True
// //     except:
// //         return False

// // def deleteStaticIPsStatusReserved(self) -> bool:
// //     try:
// //         command = ["gcloud",f"--project={self.googleProjectId}","compute","addresses","list","--format=value(name)","--filter=STATUS ~ ^RESERVED"]
// //         out = check_output(command)
// //         object_list = out.decode('utf-8')
// //         object_split_list = object_list.splitlines()
// //         for objectId in object_split_list:
// //             self.helper_deleteOrphanObject(objectId, "addresses")
// //         print(u"\n\n \u001b[48;5;196" + "m " +"IF YOU SEE ERRORS HERE, YOU NEED TO DELETE MANUALLY!\u001b[0m")
// //         print(f"\nhttps://console.cloud.google.com/networking/addresses/list?authuser=2&project={self.googleProjectId}&addressesTablesize=50\n")
// //         return True
// //     except:
// //         return False

// // def deleteTargetPools(self) -> bool:
// //     try:
// //         command = ["gcloud",f"--project={self.googleProjectId}","compute","target-pools","list","--format=value(name)"]
// //         out = check_output(command)
// //         object_list = out.decode('utf-8')
// //         object_split_list = object_list.splitlines()
// //         for objectId in object_split_list:
// //             self.helper_deleteOrphanObject(objectId, "forwarding-rules")
// //             self.helper_deleteOrphanObject(objectId, "target-pools")
// //             self.helper_deleteOrphanObject(objectId, "health-checks")
// //         print(u"\n\n \u001b[48;5;21" + "m " +"IF YOU SEE ERRORS HERE, >>>>> FALSE POSITIVE <<<<<\u001b[0m")
// //         print(f"\nhttps://console.cloud.google.com/net-services/loadbalancing/loadBalancers/list?project={self.googleProjectId}\n")
// //         return True
// //     except:
// //         return False

//     //TODO: fn generate_yaml_ingress_files
//     //TODO: fn generate_yaml_service_files
//     //TODO: fn generate_yaml_authentication_files
//     //TODO: fn generate_yaml_storage_files
//     //TODO: fn generate_yaml_dns_files
//     //TODO: fn delete_yaml_ingress_files
//     //TODO: fn delete_yaml_service_files
//     //TODO: fn delete_yaml_authentication_files
//     //TODO: fn delete_yaml_storage_files
//     //TODO: fn delete_yaml_dns_files


// }
