// criando modulos para o rust 

// 1 exemplo de modulo 

mod Ver_fatura_do_Cliente{
    pub fn fatura() { // Tenho que por como função Publica 

        println!(" O valor da fatura é 1.2212.00 R$ ");
        ver_fatura_teste(); 
    }
    fn ver_fatura_teste(){
        println!(" Ok essa é a fatura "); 
    }
}


// 2 exemplo de modulo 

mod exemplo2 {

    // Eu posso criar dois modulos  dentro de um modulo. Obs: Eu posso chamar um modulo dentro de outro, Observe abaixo

    pub mod Ler_Fatura{
        pub fn ler_farturaJa_paga() {

            println!(" Essa fatura ja esta paga rsrs.");
        }
    }

    // eu posso chamar o modulo ler_fatura porque está no mesmo scopo o no mesmo pai 

   pub mod Ver_se_pagou {


        pub fn ver() {
       let a =  crate::exemplo2::Ler_Fatura::ler_farturaJa_paga();
        }
    }


}

// Exemplo 3 

fn teste () {
    println!("K ");
}

mod carro{

   pub fn ver_Carro(){

        println!(" Carro ");
        super::teste();
    }
}

// EXEMPLO 4 

// fazendo o uso do use 

mod lanby{
    pub fn conectar(){

        println!(" Conectei ");
        let a = "ss";
    }
}
use lanby::conectar;

// exemplo 5 usando o Use 

mod lanby2{
    pub fn conectar2(){

        //println!(" Conectei ");


     

     fn teste(){
            println!(" OLa esse foi um teste");
        }

        teste(); 
    }
}
use lanby2::conectar2 as p; 
fn main() { 
    let a = carro::ver_Carro();


    println!("{:?}", a);

    conectar();
    p();

}