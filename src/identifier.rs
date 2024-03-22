use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct User {
    pub name : String,
    pub password: String,
    pub folders : Vec<String>
}

pub fn check_user(name : &str , password : &str ,u : &mut Option<User>) {
    let content = fs::read_to_string("user.json").unwrap();
    let users = serde_json::from_str::<Vec<User>>(&content).unwrap();

    for user in users.iter() {
        if user.name == name && user.password == password {
            *u = Some(user.clone());
            break;
        }
    }
}


