use std::{env, io::{prelude::*, BufReader}, io, net::{TcpStream}};
use std::fs::File;
use std::net::TcpListener;
use std::path::Path;
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

    let config_path = args.get(1).expect("Please provide a config file path as first argument");

    if config_path.is_empty() {
        println!("Please provide a config file path");
        return;
    }

    let mut file_content = String::new();
    File::open(config_path).expect("File not found").read_to_string(&mut file_content).unwrap();
    let docs = YamlLoader::load_from_str(&mut file_content).expect("Invalid YAML");
    let collected_docs: Vec<Config> = docs.iter().map(|doc| {
        return parse_config(&doc);
    }).collect();

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
        listener.set_nonblocking(true).expect("Cannot set non-blocking");
        println!("Server started at {}", config.base_url.as_str());
        listeners.push(listener);
    }

    loop {
        for listener in listeners.iter() {
            for (index, stream) in listener.incoming().enumerate() {
                let config = collected_docs.get(index).unwrap();
                match stream {
                    Ok(s) => {
                        // do something with the TcpStream
                        handle_stream(s, config);
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
    let method_header = http_request.get(0).unwrap().split_whitespace().collect::<Vec<&str>>();
    let method = method_header.get(0).unwrap().to_string();
    let file = method_header.get(1).unwrap().to_string();
    if method != "GET" {
        let response = format!("HTTP/1.1 405 METHOD NOT ALLOWED\r\n\r\n{}\r\n", "Method Not Allowed");
        stream.write_all(response.as_bytes()).unwrap();
        return;
    }

    let mut content: String = String::new();
    let mut index: String = String::from("index.html");
    if file != "/" {
        index = file;
    }
    let path = format!("{}{}", config.root, &index);
    let file_exists = Path::new(&path).exists();
    if file_exists == false {
        let response = format!("HTTP/1.1 404 NOT FOUND\r\n\r\n{}\r\n", "Page Not found");
        stream.write_all(response.as_bytes()).unwrap();
        return;
    }
    let mut file = File::open(path).unwrap();
    file.read_to_string(&mut content).unwrap();
    println!("Request: {http_request:#?}");
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}\r\n", content);

    stream.write_all(response.as_bytes()).unwrap();
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_method() {
        assert!(false)
    }
}
