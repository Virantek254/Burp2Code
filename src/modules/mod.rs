// Modules
mod java;
mod javascript;
mod python;
mod rust;
mod powershell;

// Libs
use std::fs::File;
use std::path::Path;
use std::io::BufRead;

pub fn check(req: &String, code: &String) {

    let pathfile = Path::new(&req);
    let file = File::open(pathfile).unwrap();
    let bufreader = std::io::BufReader::new(file);
    let lines = bufreader.lines();

    match code.as_str() {
        "python" => {
            // coming soon
            println!("Converting to Python");
            python::format();
        }
        "powershell" => {
            // coming soon
            println!("Converting to Powershell");
            powershell::format();
        }
        "javascript" => {
            // coming soon
            println!("Converting to JavaScript");
            javascript::format();
        }
        "java" => {
            // coming soon
            println!("Converting to Java");
            java::format();
        }
        "rust" => {
            println!("Converting to Rust");
            rust::format(lines);
        }
        _ => {
            println!("Unknown programming language");
        }
    }
}