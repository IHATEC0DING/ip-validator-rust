use ip_validator::is_valid_ip;
use std::io::{self, Write};
fn main(){
    loop{
    print!("Enter an IP address: ");
    io::stdout().flush().unwrap();
    let mut ip = String::new();
    io::stdin().read_line(&mut ip).unwrap();
    let ip = ip.trim();
    if is_valid_ip(ip) {
        println!("{} IP adress is valid!", ip);
    } else {
        println!("{} IP adress is not valid!", ip);
    }  
    }      
}