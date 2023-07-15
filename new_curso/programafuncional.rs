use std::time::Duration;
use std::thread::sleep; 

fn responder(a: i32) -> i32 {

    println!("Lanby");
    sleep( Duration::from_secs(a as u64 ));
    a

}

fn main() {

    println!(" Programa funcional ");

    let a = r#"{
        nome="Lanby", 
        sobrenome="caio"
    } "#;

    println!("{}", a);
    
    let a: i32 = 10;
    dbg!(a);
}