use std::{env, fs, io::{prelude::*, BufReader}, net::{TcpStream}, thread};
use std::fs::File;
use std::net::TcpListener;
use std::path::Path;
use std::time::Duration;
use yaml_rust2::{Yaml, YamlLoader};
use webserver_rust::ThreadPool;

#[derive(Hash, Debug)]
struct Config {
    base_url: String,
    root: String,
}

fn parse_config(hash: &Yaml) -> Config {
    let base_url = hash["base_url"].as_str().unwrap().to_string();
    let root = hash["root"].as_str().unwrap().to_string();
    Config {
        base_url,
        root,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config_path = args.get(1).unwrap();

    if config_path.is_empty() {
        println!("Please provide a config file path");
        return;
    }

    let mut file_content = String::new();
    File::open(config_path).expect("File not found").read_to_string(&mut file_content).unwrap();
    let docs = YamlLoader::load_from_str(&mut file_content).expect("Invalid YAML");
    let collected_docs: Vec<Config> = docs.iter().enumerate().map(|(index, doc)| {
        return parse_config(&doc);
    }).collect();

    let pool = ThreadPool::new(collected_docs.len());
    println!("Config: {:?}", collected_docs);

    for config in collected_docs {
        let exists = Path::new(&config.root).exists();
        if !exists {
            println!("Root path does not exist {}", config.root);
            return;
        }
        pool.execute(|| {
            handle_connection(config);
        });
    }
    println!("Shutting down.");
}


fn handle_connection(config: Config) {
    let listener = TcpListener::bind(config.base_url.as_str()).unwrap();

    println!("Server started at {}", config.base_url.as_str());

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_stream(stream)
    }
}

fn handle_stream(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    println!("{:?}", buf_reader);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    let method = http_request.get(0);
    println!("Method: {method:#?}");

    println!("Request: {http_request:#?}");
    let response = "HTTP/1.1 200 OK\r\n\r\nHello\r\n";

    stream.write_all(response.as_bytes()).unwrap();
}
