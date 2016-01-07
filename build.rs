use std::path::Path;
use std::fs::copy;
use std::env;

const RKT_FNAME: &'static str = "rust_functions.rkt";

fn main() {
    let out_var = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_var);
    let target_dir = out_dir.join("..").join("..").join("..");
    let manifest_var = env::var("CARGO_MANIFEST_DIR").unwrap();
    let manifest_dir = Path::new(&manifest_var);
    let src_dir = manifest_dir.join("src");
    let rkt_src_path = src_dir.join(RKT_FNAME);
    let rkt_out_path = target_dir.join(RKT_FNAME);
    copy(rkt_src_path, rkt_out_path).expect(&format!("Failed to copy {}", RKT_FNAME));
}
