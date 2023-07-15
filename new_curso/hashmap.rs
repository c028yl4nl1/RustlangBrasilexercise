
//Iniciando um HashMap 

use std::collections::HashMap;

fn main(){

    println!("Aula sobre HashMap");

    // Primeira coisa que descobrir sobre hashmap  è que é igual ao dicionario 
    
    {

        // Estrutura Basica de um HashMap
        let mut Livros = HashMap::new(); // Aqui Estou criando um HashMap Vazio 

        println!("{:?}", Livros); // ver como está 
        Livros.insert( // Inserindo valor 
            String::from("Nome"), // Chave  
            vec![ // Depois da virgula é o valor 
                String::from(" Teste"),
            ]
        );
        println!("{:?}", Livros["Nome"]);

        // fIM DO pRIMEIRO EXEMPLO 

    }

    // Exemplo 2 
    {
        let chave = String::from("Lanby");

        let valor = String::from(" Come tu ");

        let mut Hash = HashMap::new();

        Hash.insert(chave , valor); // A chave e o Valor foi eliminado estava na heap 


        println!("{:?}", Hash);

        let chave = String::from("Lanby2");
        let valor = String::from("Come tu");

        Hash.insert(chave, valor);


        println!("{:?}", Hash);

        // Pegando um valor de uma chave

        let result = Hash.get("Lanby2"); 
        match result {
            Some(_) => println!(" Tem valor "),
            None => println!(" Não tem valor "),
        }

        println!("{:?}", Hash);


        // Fazendo um exemplo com chave e valor no HashMap 

        for (chave , valor) in Hash {
            println!("Chave {} Valor {}" , chave , valor);
        }

    }


   



}