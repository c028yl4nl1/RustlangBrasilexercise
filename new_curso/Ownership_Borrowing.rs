


fn main() {

    // Primeiro exemplo 

    // 
    let exemplo_1_errado = {
        let exemplo_1 = String::from("Valor alocada na heap ");
       // exemplo_fn(exemplo_1); erro de ownership

        fn exemplo_fn (valor: String){

            println!(" Se eu tenta usar a variavel exemplo_1 vai dar erro de emprestimo {} ", valor);
            
            
             }

        println!("{}", exemplo_1);

        



    }; 

    
    let exemplo_2_certo = {

        let String_ = String::from( " Valor alocado na heap ");

        pega_valor(&String_); // não estou alocando todo o valor na heap , estou apenas fazendo um Borrowing  
        fn pega_valor(valor: &String) { // Peguei uma referencia da memoria heap um emprestimo 

            println!("{}", valor);
        }

        println!("{}", String_);
    };


    println!("{:?}", exemplo_2_certo);

    // mudar valor pela a referecia 
{
    let mut x = String::from("Ola");
  
    
    let ref_ = &x;

    // aqui não funciona mudar(&mut x);
    println!(" Valor: {}", ref_);
    mudar(&mut x);  // aqui funciona 
    

    fn mudar(valor: &mut String) {
        valor.push_str("kk");

    }

    
    println!(" Valor: {}",x);
}

}