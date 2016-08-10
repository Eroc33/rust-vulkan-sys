#![feature(slice_patterns)]
// build.rs

use std::env;

fn main() {
  let vk_path = env::var("VK_SDK_PATH").unwrap();
  let target = env::var("TARGET").unwrap();
  let target_split:Vec<_> = target.split("-").collect();
  let bitage = match target_split.as_slice(){
      &["i686","pc",..] => 32,
      &["x86_64","pc",..] => 64,
      _ => panic!("I don't know how to link the vulkan sdk on this system!"),
  };
  let folder = if bitage == 32{
    "Source/lib32"
  }else{
    "Source/lib"
  };
  println!("cargo:rustc-link-search=native={}/{}",vk_path,folder);
  println!("cargo:rustc-link-lib=static=vulkan-1");
}
