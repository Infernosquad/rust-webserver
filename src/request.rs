pub mod request {
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;
    #[derive(Hash, Debug)]
    pub struct Config {
        pub base_url: String,
        pub root: String,
    }

    pub struct Request {
        pub method: String,
        pub file: String,
    }

    pub fn process_request(request: &Request, config: &Config) -> String {
        if request.method != "GET" {
            let response = format!(
                "HTTP/1.1 405 METHOD NOT ALLOWED\r\n\r\n{}\r\n",
                "Method Not Allowed"
            );
            return response;
        }

        let mut content: String = String::new();
        let mut index: String = String::from("/index.html");
        if request.file.trim() != "/" {
            index = request.file.trim().to_string();
        }
        let path = format!("{}{}", config.root, &index);
        let file_exists = Path::new(&path).exists();
        if file_exists == false {
            let response = format!("HTTP/1.1 404 NOT FOUND\r\n\r\n{}\r\n", "Page Not found");
            return response;
        }
        let mut file = File::open(path).unwrap();
        file.read_to_string(&mut content).unwrap();
        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}\r\n", content);

        return response;
    }
}

