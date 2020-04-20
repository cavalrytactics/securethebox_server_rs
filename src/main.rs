use securethebox_server_rs::controllers::travis;
fn main() {
   let _ = travis::decrypt_tar_secrets();
}