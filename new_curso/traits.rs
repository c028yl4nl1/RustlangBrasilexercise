
#[derive(Debug)]
struct Cliente{
    nome: &'static str, 
    forncedor:  &'static str, 
}

trait Pessoa {
    fn nome(&self) -> &str; 
}

impl Pessoa for Cliente {
    fn nome(&self) -> &str {
        let a = format!("OLa meu nome : {}",self.nome);
        println!("{}", a);

        "dss"


    }
}

fn main() {

    println!(" Hoje vamos falar sobre traits ");

    // Trait é meio que uma interface 

    let c1 = Cliente{nome: "Lanby", forncedor:"Binga"};

    println!("{:#?}", c1);
}