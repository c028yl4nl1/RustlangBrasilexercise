
use std::rc::Rc;

pub trait Mensageiro {
    fn enviar(&self, msg: &str) {

    }
    
}

pub struct Limit<'a T: Mensageiro>{
    msgs: &'a T,
    Limit: isize,
}
fn main() {
    println!(" Hoje vamos aprender sobre o refcel ");

    //Rc<T> serve apenas para ver um valor na heap

    let list = Rc::new(vec![1,2,34,4,5,6]);

    println!("{:?}", list);


    // Rc<T> é sigle thread não multi thread 


    //não entendir 

}