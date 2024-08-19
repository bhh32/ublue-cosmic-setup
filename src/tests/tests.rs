
use std::process::Command;
use crate::logic::*;

#[test]
fn test_get_cur_usr_id() {
    let test_usr_id = get_cur_usr_id();

    let test_cmd = Command::new("id")
        .arg("-u")
        .output()
        .expect("The current user's ID could not be returned!");

    let test_cmd_output = match String::from_utf8(test_cmd.stdout).unwrap().trim().parse::<u32>() {
        Ok(test_output) => test_output,
        Err(e) => {
            eprintln!("Test Command Output: {e}");
            0
        }
    };

    assert_eq!(test_usr_id, test_cmd_output);
}

#[test]
fn test_get_cur_usr_name() {
    let test_usr_name = get_cur_usr_name();
    let test_cmd = Command::new("whoami")
        .output()
        .expect("get_cur_usr test command failed!");

    let test_cmd_output = String::from_utf8(test_cmd.stdout).unwrap().trim().to_string();

    assert_eq!(test_usr_name, test_cmd_output);
}

#[test]
fn test_get_cur_usr() {
    let cur_user = get_current_user();
    let test_usr_name = get_cur_usr_name();
    let test_usr_id = get_cur_usr_id();

    let test_usr: User = User::new(
            test_usr_id,
            test_usr_name,
        );

    assert_eq!(cur_user.uid, test_usr.uid);
    assert_eq!(cur_user.uname, test_usr.uname);
}

#[test]
fn test_setup_usr_skel() {
    match setup_usr_skel() {
        (Some(SetupSuccess::SKEL), None) => assert!(true),
        _ => assert!(false),
    }
}