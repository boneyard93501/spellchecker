
use marine_rs_sdk::{marine, MountedBinaryStringResult};

pub fn main() {}

#[marine]
pub fn aspell(cmd: Vec<String>) -> MountedBinaryStringResult {
    aspell(cmd)
}

#[marine]
#[link(wasm_import_module = "host")]
extern "C" {
    fn aspell(cmd: Vec<String>) -> MountedBinaryStringResult;
}