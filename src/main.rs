mod request;
mod async_lib;
mod tests;

use crate::request::request::{Config, process_request, Request};
use std::fs::File;
use std::net::TcpListener;
use std::path::Path;
use std::{
    env, io,
    io::{prelude::*, BufReader},
    net::TcpStream,
};
use yaml_rust2::{Yaml, YamlLoader};
use crate::async_lib::async_lib::ThreadPool;


fn parse_config(hash: &Yaml) -> Config {
    let base_url = hash["base_url"].as_str().unwrap().to_string();
    let root = hash["root"].as_str().unwrap().to_string();
    Config { base_url, root }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config_path = args
        .get(1)
        .expect("Please provide a config file path as first argument");

    if config_path.is_empty() {
        println!("Please provide a config file path");
        return;
    }

    let mut file_content = String::new();
    File::open(config_path)
        .expect("File not found")
        .read_to_string(&mut file_content)
        .unwrap();
    let docs = YamlLoader::load_from_str(&mut file_content).expect("Invalid YAML");
    let collected_docs: Vec<Config> = docs
        .iter()
        .map(|doc| {
            return parse_config(&doc);
        })
        .collect();

    let pool = ThreadPool::new(collected_docs.len());
    println!("Config: {:?}", collected_docs);

    let mut listeners = vec![];
    for config in &collected_docs {
        let exists = Path::new(&config.root).exists();
        if !exists {
            println!("Root path does not exist {}", config.root);
            return;
        }
        let listener = TcpListener::bind(config.base_url.as_str()).unwrap();
        listener
            .set_nonblocking(true)
            .expect("Cannot set non-blocking");
        println!("Server started at {}", config.base_url.as_str());
        listeners.push(listener);
    }

    loop {
        for (index, listener) in listeners.iter().enumerate() {
            for stream in listener.incoming() {
                match stream {
                    Ok(s) => {
                        let config = collected_docs.clone();
                        // do something with the TcpStream
                        pool.execute(move || {
                            handle_stream(s, config.get(index).unwrap());
                        });
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        break;
                    }
                    Err(e) => println!("encountered IO error: {e}"),
                }
            }
        }
    }
}

fn handle_stream(mut stream: TcpStream, config: &Config) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .filter(|result| result.is_ok())
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    let method_header = http_request
        .get(0)
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>();
    let method = method_header.get(0).unwrap().to_string();
    let file = method_header.get(1).unwrap().to_string();
    let request = Request {
        method: method.clone(),
        file: file.clone(),
    };

    return stream.write_all(process_request(&request, &config).as_bytes()).unwrap();
}
