fn main() {

    println!("Vamos falar sobre lifetime");
    let a = String::from("Lanby");

    lifetimeExemplo(&a);

    println!("{}", a);

    let r: i32 ; 
    {
        let a = 10;
        r= a;

    }

  
    println!("{:?}", r);

}


fn lifetimeExemplo<'a>(a: &'a str) -> &'a str{
    let b = a.len();
    let c = a;

    a
}
fn teste() -> &'static i32 {
    println!("Teste");

    &1 
}

