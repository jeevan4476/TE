use std::io::{self,Read};
use crossterm::terminal::{enable_raw_mode,disable_raw_mode};
fn main() {
    match enable_raw_mode() {
        Ok(()) => println!("Raw mode Enalbed"),
        Err(e) => {
            eprintln!("Failed to enable raw mode: {}", e);
            return;
        }
    }
    for i in io::stdin().bytes(){
        let c = i.unwrap() as char;
        println!("{}",c);
        if c == 'q' {
            if let Err(e) = disable_raw_mode() {
                eprintln!("Error disabling raw mode: {}", e);
            }
            break;
        }
    }
    
}
