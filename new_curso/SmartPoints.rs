fn main() {

    println!(" smart Points ");

    // ponteiro ou apontador é um tipo de dado de uma linguagem de programação cujo valor se refere diretamente a um outro valor alocado em outra área da memória, através de seu endereço

    // na memoria stack temos que esplicitar o valor concreto  
    // na memoria heap é dinamico 

    let x = Box::new(4);

    println!("{}", x);

    let a = "lanby come cu".as_bytes();
    

    let y =  &a;
    drop(a);
   

    println!("{:?}", y);

    // não aprendir , realidade.
}