
use std::fs::File;
use std::io::{BufReader, Lines};
use regex::Regex;

pub fn format(lines: Lines<BufReader<File>>) {

    let mut method = String::new();
    let mut host = String::new();
    let mut path = String::new();

    for (linenumber, line) in lines.enumerate() {
        let line = line.unwrap();
        if linenumber == 0 {
            let re = Regex::new(r"^([A-Z]+)\s+([^\s]+)\s+(HTTP/[0-9.]+)$").unwrap();
            if let Some(captures) = re.captures(&*line) {
                method = captures.get(1).map_or("".parse().unwrap(), |m| m.as_str().parse().unwrap());
                path = captures.get(2).map_or("".parse().unwrap(), |m| m.as_str().parse().unwrap());
                println!("\
// Client Initiation
let client = Client::new()\n");
            }
            println!("//HeaderMap\nlet mut headermap = HeaderMap::new();");
        }
        if linenumber != 0 {
            let re = Regex::new(r"^([\w-]+):\s*(.+)$").unwrap();
            if let Some(caps) = re.captures(&*line) {
                let header = caps.get(1).unwrap().as_str();
                let value = caps.get(2).unwrap().as_str();
                if header == "Host" {
                    host = value.to_string();
                }
                println!("headermap.insert(\"{}\", HeaderValue::from_static(\"{}\"));", header, value);

            }
        }

    }
    println!("\n// Request
let response = client.{}(\"https://{}{}\")
    .headers(headermap)
    .send()
    .await
    .unwrap();
", method.to_lowercase(), host, path)
}
