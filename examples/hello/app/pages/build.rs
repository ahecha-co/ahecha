use etagere_build::generate_routes;
use std::{env, fs, path::Path};

fn main() {
  let out_dir = env::var_os("OUT_DIR").unwrap();
  let dest_path = Path::new(&out_dir).join("routes.rs");
  fs::write(&dest_path, generate_routes("")).unwrap();
  println!("cargo:rerun-if-changed=src/*");
}
