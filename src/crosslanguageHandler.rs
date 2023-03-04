use std::collections::HashMap;
use std::process::Command;

use json;
use json::codegen::PrettyWriterGenerator;

pub struct crosslanguageHandler {
    pub routeValue: String,
    pub result : String,
}

impl crosslanguageHandler {
    pub fn new() -> crosslanguageHandler {
        crosslanguageHandler {
            routeValue: String::new(),
            result: String::new(),
        }
    }
    pub fn handle(&mut self, route: &HashMap<String, String>, user_arg: &HashMap<String, String>, path: &String) {
        // /anything -> {"file": "createAccount.py", "args": ["username", "password", "email"], "exec": "python3"}
        
        //get the route value
        self.routeValue = route.get(path).unwrap().to_owned();
        //parse the route value
        let routeValue = json::parse(&self.routeValue).unwrap();
        //get the file name
        let file = routeValue["file"].to_string();
        //get the args
        let args = routeValue["args"].members();
        //get the exec
        let exec = routeValue["exec"].to_string();
        //create a command
        let mut command = Command::new(exec);
        //add the file name to the command
        command.arg(file);
        //add the args to the command
        for arg in args {
            // user_arg is a hashmap of the arguments passed by the user <key, value>
            
            if user_arg.get(&arg.to_string()).is_none() {
                println!("{} is not defined", arg);
                continue;
            }
            let arg = user_arg.get(&arg.to_string()).unwrap();
            
            command.arg(arg.to_string());
        }
        //run the command
        println!("command: {:?}", command);
        let output = command.output().expect("failed to execute process");
        let stdout = String::from_utf8_lossy(&output.stdout);
        let result = command.output().unwrap();
        if result.status.success() {
            // println!("success");
        } else {
            println!("error");
            println!("{}", String::from_utf8_lossy(&result.stderr));
        }
        //get the output
        self.result = String::from_utf8(output.stdout).unwrap();
        
    }
}