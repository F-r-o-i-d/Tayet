mod firewall;
mod parser;
mod headersBuilder;
mod crosslanguageHandler;
//import socket
use std::net::TcpListener;
//import thread
use std::thread;
//import stream
use std::net::TcpStream;
//import io
use std::io::prelude::*;
//import file
use std::fs::File;
//import env
use std::env;
//import path
use std::path::Path;
//import path buf
use std::path::PathBuf;
use std::collections::HashMap;

use json::codegen::PrettyWriterGenerator;

//create global variable

fn main() {
    //show actual path
    println!(
        "The current directory is: {}",
        std::env::current_dir().unwrap().display()
    );
    let setting = parser::get_setting();
    let route = parser::get_route();

    for (key, value) in route {
        println!("routing {} to {}", key, value);
    }
    //setup the listener
    let host = setting.get("host").unwrap();
    let port = setting.get("port").unwrap();
    let listener = TcpListener::bind(format!("{}:{}", host, port)).unwrap();
    //loop through the listener
    for stream in listener.incoming() {
        //handle the stream
        match stream {
            Ok(stream) => {
                //create a thread to handle the stream
                thread::spawn(move || {
                    //handle the stream
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {

    let route = parser::get_route();
    let setting = parser::get_setting();
    //create a buffer to store the data
    let mut buffer = [0; 512];
    //read the data from the stream
    stream.read(&mut buffer).unwrap();
    //print the data
    //get the request
    let request = String::from_utf8_lossy(&buffer[..]);
    //get the request method
    let method = request.split_whitespace().next().unwrap();
    //get the request path
    //handle arguments
    let args = request.split_whitespace().nth(1).unwrap().split("?").nth(1);
    //create an hashmap to store the arguments
    let mut arguments = HashMap::new();
    if args.is_some() {
        let args = args.unwrap();
        for arg in args.split("&") {
            let arg = arg.split("=").collect::<Vec<&str>>();
            arguments.insert(arg[0].to_string(), arg[1].to_string());
        }
        
    }
    println!("{:?}", arguments);
    let path = request.split_whitespace().nth(1).unwrap().split("?").next().unwrap();
    //get the request version
    let version = request.split_whitespace().nth(2).unwrap();
    //get the request header
    let header = request.split("\r\n\r\n").next().unwrap();
    //get the request body
    if request.split("\r\n\r\n").nth(1).is_none() {
        println!("No body");
    } else {
        let body = request.split("\r\n\r\n").nth(1).unwrap();
        
    }
    //get the path from the route
    //check if the path is in the route
    println!("path: {}", path);

    let serverName = setting.get("serverName").unwrap();
    let mut headers = headersBuilder::headersBuilder::new();
    let mut response = String::new();
    if firewall::is_request_legitimate(&request, parser::get_route()) && setting.get("firewall").unwrap() == "true" {
        //create a response7
        let mut file = File::open(setting.get("firewallAlertFile").unwrap());
        //handle if file not found
        if file.is_err() {
            println!("[!] - File not found : {}", setting.get("firewallAlertFile").unwrap());
            return;
        }
        let mut contents = String::new();
        file.unwrap()
            .read_to_string(&mut contents)
            .expect("something went wrong reading the file");
        //create a response
        headers.add_header("HTTP/1.1", "403 Forbidden".to_string());
        headers.add_header("Content-Length", contents.len().to_string());
        headers.add_header("Server", serverName.to_string());
        headers.add_header("Content-Type", "text/html".to_string());
        response = format!("{}{}", headers.build(), contents);
        
        
        
    }
    if route.contains_key(path) {

        let mut contents = String::new();
        
        //get the file path
        let file_path = route.get(path).unwrap();
        if file_path.contains("{"){
            println!("crosslanguage");
            let mut corsslanguageHandler = crosslanguageHandler::crosslanguageHandler::new();
            //convert arguments to HashMap
            
            corsslanguageHandler.handle(&route,&arguments, &path.to_string());
            let mut output = corsslanguageHandler.result;
            if output.len() == 0 {
                output = "Empty output".to_string();
            }
            contents = output;

        } else {
            println!("Not crosslanguage");
            //read the file
            let mut file = File::open(file_path);
            //handle if file not found
            if file.is_err() {
                println!("[!] - File not found : {}", file_path);
                return;
            }
            file.unwrap()
                .read_to_string(&mut contents)
                .expect("something went wrong reading the file");
            //create a response
        }
        headers.add_header("HTTP/1.1", "200 OK".to_string());
        headers.add_header("Content-Length", contents.len().to_string());
        headers.add_header("Server", serverName.to_string());
        headers.add_header("Content-Type", "text/html".to_string());
        response = format!("{}{}", headers.build(), contents);
        
        
    } else {
        //create a response
        response = format!("{} 404 Not Found\r\nContent-Length: 0\r\n\r\n", version);
    }
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
