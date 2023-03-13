use std::collections::HashMap;

pub struct headersBuilder {
    headers: HashMap<String, String>,
}


impl headersBuilder {
    pub fn new() -> headersBuilder {
        headersBuilder {
            headers: HashMap::new(),
        }
    }

    pub fn add_header(&mut self, key: &str, value: String) {
        self.headers.insert(key.to_owned(), value);
    }
    pub fn remove_header(&mut self, key: &str) {
        self.headers.remove(key);
    }
    pub fn reverse(&mut self) {
        //insert the headers in the reverse order
        let mut headers = HashMap::new();
        for (key, value) in &self.headers {
            headers.insert(key.to_owned(), value.to_owned());
        }
    }
    pub fn get_headers(&self) -> &HashMap<String, String> {
        return &self.headers;
    }
    pub fn empty(&mut self) {
        self.headers.clear();
    }
    fn check_integrity(&mut self) -> bool {
        if !self.headers.contains_key("Server") {
            return false;
        }
        if !self.headers.contains_key("Content-Length") {
            return false;
        }
        if !self.headers.contains_key("Content-Type") {
            return false;
        }

        
        return true;
    }

    pub fn build(&mut self) -> String {
        let mut headers = String::new();
        let mut finalHeaders = String::new();
        if !self.check_integrity() {
            //make an error and stop the program
            panic!("Headers miss some important headers");
        }
        //add http version header and status code
        let mut headersVersion = String::new();
        for (key, value) in &self.headers {
            if (key.starts_with("HTTP/")) {
                headersVersion = format!("{} {}\r\n", key, value);
                break;
            }
        }
        finalHeaders.push_str(&headersVersion);
        for (key, value) in &self.headers {
            if (key.starts_with("HTTP/")) {
                continue;
            }
            finalHeaders.push_str(&format!("{}: {}\r\n", key, value));
        }
        finalHeaders.push_str("\r\n");
        return finalHeaders;
    }
}
