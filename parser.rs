use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
//define a function to get the setting from the setting file

pub fn get_setting() -> HashMap<String, String> {
    let mut setting = HashMap::new();
    let mut file = File::open("config.conf").expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("something went wrong reading the file");

    for line in contents.lines() {
        let mut key_value = line.split("=");
        let key = key_value.next().unwrap().to_owned();
        let value = key_value.next().unwrap().to_owned();
        setting.insert(key, value);
    }

    return setting;
}

pub fn get_route() -> HashMap<String, String>{
    //create an dict to store the setting
    let mut route = HashMap::new();
    //read the setting file
    let mut file = File::open("route.conf").expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("something went wrong reading the file");
    //split the setting file into lines
    let lines = contents.lines();
    //loop through the lines
    for line in lines {
        //split the line into key and value
        let mut key_value = line.split("->");
        //get the key
        let key = key_value.next().unwrap().to_owned();
        //get the value
        let value = key_value.next().unwrap().to_owned();
        //insert the key and value into the dict
        route.insert(key, value);
    }
    //return the dict
    return route;
}