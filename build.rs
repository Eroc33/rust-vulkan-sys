// build.rs

use std::env;

fn main() {
  let vk_path = env::var("VK_SDK_PATH").unwrap();
  println!("cargo:rustc-link-search={}/Bin",vk_path)
}
