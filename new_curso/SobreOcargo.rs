use std::env::args;
use std::process;
use rayon::prelude::*;


#[derive(Debug)]
struct Config{
    Host: String, 
    Port: u64,
    Threads: u64
    

}

impl Config {
    pub fn new(Host: String,Port: u64, Num_Threads: u64 ) -> Config {
        Config{Host: Host, Port: Port, Threads: Num_Threads}
    }
}
fn argsRecev() -> Result<Vec<String>, &'static str> {
    let args_send: Vec<String> = args().collect();
    if args_send.len() < 4{ // [0] path caminho [1] Host [2] Port
        return Err("Modo de uso: ./bin Host Port Threands");
        //process::exit(1);
    }
    Ok(args_send)
}
fn main(){

    println!("Dos Multithread");

    let argsArray = match argsRecev(){
        Ok(array) => array,
        Err(valor) => {
            println!("{}", valor);
            process::exit(1);   
        }
    }; 
    let a = Config::new(String::from("127.0.0.1"), 10 as u64, 22 as u64 );

    let mut a = String::from(" Teste Multithread");


}

fn printar(x: String) {
    println!("{}", x);
}
 
fn mudar_mais() {
    for c in 0..20{
        println!("Numeber : {}", c);
    }
}
