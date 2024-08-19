mod cli;
pub mod logic;
pub mod errors;

#[cfg(test)]
mod tests;

use logic::*;

fn main() {
    println!("{}: {}", get_cur_usr_name().trim(), get_cur_usr_id());
}
