#![allow(non_snake_case)]
use marine_rs_sdk::{marine, MountedBinaryStringResult};

use serde::Serialize;
use serde_json;
use ispell::{ SpellLauncher, SpellChecker, IspellError};

pub fn main() {}


fn get_checker(dict: String) -> Result<SpellChecker, String> {
    
    let checker = SpellLauncher::new()
        // .aspell()
        // .command("aspell")
        .hunspell()
        .dictionary(&dict)
        .launch();
    match checker {
            Ok(s) => Ok(s),
            Err(e) => Err(format!("Failed ot launch chcker: {}", e)),
        }       
}


#[marine]
pub struct CheckerResult {
    pub stdout: Vec<String>,
    pub stderr: String,
}
#[marine]
#[derive(Serialize)]
pub struct SpellError {
    pub misspelled: String,
    pub position: u32,
    pub suggestions: Vec<String>
}

impl SpellError {
    fn from_IspellError(e: IspellError) -> Self {
        SpellError {
            misspelled: e.misspelled,
            position: e.position as u32,
            suggestions: e.suggestions
        }
    }

    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}


#[marine]
pub fn checker(text: String, dict: String) -> CheckerResult {
    let c_result = get_checker(dict);
    let mut checker:SpellChecker = match c_result {
        Ok(c) => c,
        Err(e) => { return CheckerResult {stdout: vec!(), stderr: e};},
    };

    let errors = match checker.check(&text) {
        Ok(errs) => errs,
        Err(e) => {return CheckerResult { stdout: vec!(), stderr: e.to_string()};},
    };

    let mut result:Vec<String> = Vec::new();
    for e in errors {
        result.push(SpellError::from_IspellError(e).to_string());
    }

    
    CheckerResult {stdout: result, stderr: "".to_owned()}
}

/* 
#[marine]
 #[link(wasm_import_module = "aspell_adapter")]
 extern "C" {
     pub fn spell_check(cmd: Vec<String>) -> MountedBinaryStringResult;
 }
 */