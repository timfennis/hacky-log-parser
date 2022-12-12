use std::{env, fs::File, io::{BufReader, BufRead}};

use substring::Substring;

fn main() {
    let args: Vec<String> = env::args().collect();
    let start_token = "http.url_details.path";
    let end_token = "request_length";
    if let Some(file) = args.get(1) {
        let dirs = std::fs::read_dir(file).expect("Directory not found");

        for dir in dirs {
            let dir = dir.expect("Cannot get dir entry");
            let path = dir.path();
            let path = path.to_str().expect("Cannot get path from path");
            
            let file = File::open(path).expect("Unable to open file");
            let reader = BufReader::new(file);

            for line in reader.lines() {
                let line = line.expect("Unable to read line");
                let start = line.find(start_token);
                let end = line.find(end_token);
                
                match (start, end) {
                    (Some(start), Some(end)) => {

                        let token = line.substring(start + start_token.len() + 3, end - 3);
                        println!("{}", token);
                    },
                    _  => {

                    }
                }
            }
        } 
    }
}