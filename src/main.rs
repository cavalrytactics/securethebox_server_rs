use securethebox_server_rs::controllers::travis;
fn main() {
   let _ = travis::tar_decompress_secrets_directory();
   // let _ = travis::decrypt_tar_secrets();
}