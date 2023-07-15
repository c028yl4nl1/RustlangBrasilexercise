
#[derive(Debug)]
struct Produtos{
    valor_do_pedido: f64 , 
    pagamento: String, // visa , mercado pago pix entre outras
    pagou: bool , // Não é obrigatorio por a virgula no final mas o livro do rust sempre recomenda.
}

#[derive(Debug)]
struct Dupla(String, i32);

fn main() {

    // Aula sobre Struct 

    // *1 No rust eu spu obrigado a instacia todos os valores na Struct 

    let Produtos_ = Produtos{valor_do_pedido: 2.2 , pagamento: String::from("Visa"), pagou: false};
    
    let a = Produtos_.valor_do_pedido; // estou vendo o valor que foi passado  

    println!("{}",a); // estou printando o valor que foi passado 

    println!("{:#?} ", Produtos_);



    // Exemplo de mudar o valor dentro da Struct 

    let mut struct_alocar = Produtos{
        valor_do_pedido: 22 as f64 , 
        pagamento: "Cartão mastercad ".to_string() , 
        pagou: true , 
    };

    // tenhp que por o mut no Struct_alocar para mudar o valor de dentro 

    struct_alocar.pagou = false ;

    println!("{:#?} ", struct_alocar);


    // usando a Struct em forma de dupla 

    let alocar_valores = Dupla("caio".to_string() , 12.2 as i32); // Note uma coisa diferente de Struct normal Eu tenho que passar como se fosse Uma dupla


    // Acessar o valor da Dupla Struct 

    println!(" {:#?} ",alocar_valores ); // Imprimindo todo o A dupla com o Debug 

    println!("{}",alocar_valores.1 ); // Struct de dupla é igual uma dupla tenho que ver a Posição 


    //  usando implementãção 

    let impl_activer = UsandoImpl{nome:"Lanby".to_string()};


    impl_activer.teste_1();

    let a = UsandoImpl::teste_2(4); // chamando a Struct sem referecia a uma  estacia
     

    println!("{}", a);

    


}


struct UsandoImpl {
    nome:String,

}

impl UsandoImpl {

    fn teste_1 (&self) {
        println!("{}", self.nome);

    }

    // usando impl sem o uso do objeto &Self

    fn teste_2(a: i32) -> f64 {

        a as f64 * 1.2

    }
}