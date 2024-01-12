
use marine_rs_sdk::{marine, MountedBinaryStringResult};

pub fn main() {}

#[marine]
pub fn hunspell(cmd: Vec<String>) -> MountedBinaryStringResult {
    hunspell(cmd)
}

#[marine]
#[link(wasm_import_module = "host")]
extern "C" {
    fn hunspell(cmd: Vec<String>) -> MountedBinaryStringResult;
}