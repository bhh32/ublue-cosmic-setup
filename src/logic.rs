use std::process::Command;
use rpassword::prompt_password;
use crate::errors::SetupError;

#[derive(Debug)]
pub enum SetupSuccess {
    SKEL,
    HOME,
}

pub struct User {
    pub uid: u32,
    pub uname: String,
    password: Option<String>,
}

impl User {
    pub fn new(uid: u32, uname: String) -> Self {
        Self {
            uid,
            uname,
            password: None,
        }
    }
}

pub fn set_usr_password(usr: &mut User) {
    usr.password = Some(prompt_password("Enter the user's password: ").unwrap());
}

pub fn get_current_user() -> User {
    User::new(get_cur_usr_id(), get_cur_usr_name())
}

pub fn get_cur_usr_id() -> u32 {
    let usr_id = Command::new("id")
        .arg("-u")
        .output()
        .expect("The current user's ID could not be returned!");

    let ret_val = String::from_utf8(usr_id.stdout)
        .unwrap();
        
    match ret_val.trim().to_string().parse::<u32>() {
        Ok(ret) => ret,
        Err(e) => {
            eprintln!("Bad user id value, returning 0!\n{e}");
            0
        }
    }
}

pub fn get_cur_usr_name() -> String {
    let usr_name = Command::new("whoami")
        .output()
        .expect("whoami failed!");

    let ret_value = String::from_utf8(usr_name.stdout).unwrap();

    ret_value.trim().to_string()
}

pub fn setup_cur_usr_home_dir() -> (Option<SetupSuccess>, Option<SetupError>) {
    match Command::new("mkdir")
    .args(["-p", "$HOME/Documents", "$HOME/Downloads", "$HOME/Desktop", "$HOME/Pictures", "$HOME/Music", "$HOME/Videos", "$HOME/Public"])
    .status() {
        Ok(_) => (Some(SetupSuccess::HOME), None),
        Err(_) => (None, Some(SetupError::HOME)),
    }
}

pub fn setup_usr_skel() -> (Option<SetupSuccess>, Option<SetupError>) {
    match Command::new("mkdir")
    .args(["-p", "/etc/skel/Documents", "/etc/skel/Downloads", "/etc/skel/Desktop", "/etc/skel/Music", "/etc/skel/Pictures", "/etc/skel/Public", "/etc/skel/Videos"])
    .status() {
        Ok(_) => (Some(SetupSuccess::SKEL), None),
        Err(_) => (None, Some(SetupError::SKELERR))
    }
}