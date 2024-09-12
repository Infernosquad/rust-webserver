#[cfg(test)]
mod tests {
    use crate::request::request::{Config, process_request, Request};

    #[test]
    fn test_get_method() {
        let result: String = process_request(
            &Request {
                method: "GET".to_string(),
                file: "/".to_string(),
            },
            &Config {
                base_url: "127.0.0.1:8081".to_string(),
                root: "example/html".to_string(),
            },
        );
        assert_eq!(result, "HTTP/1.1 200 OK\r\n\r\n<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n    <meta charset=\"UTF-8\">\n    <title>Title</title>\n    <script src=\"/app.js\"></script>\n</head>\n<body>\n<h1>Hello world</h1>\n</body>\n</html>\n\r\n");
    }
}
