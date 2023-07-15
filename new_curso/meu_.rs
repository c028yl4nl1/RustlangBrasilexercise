use std::net::Ipv4Addr;



fn main() {
    let ip_address = Ipv4Addr::new(127, 0, 0, 1);
    

    let a = "127.0.0.1.2";
    if converter_ip(a) == true {
    } else {

        println!(" Voce digitou errado");
        panic!(); 
    }
}

fn converter_ip(ip: &str) -> bool {


    let vec_split: Vec<&str>  = ip.split(".").collect();
    
    
   
    if vec_split.len() > 4 {
        println!(" Passou do limite");
        return false 


    }
    true 







}

