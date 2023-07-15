fn main(){

    // hoje vamos falar sobre vetores 

    // Os dados do Vertores fica na Memoria heap //  Array fica na Stack 

    let mut Vertor: Vec<String> = Vec::new(); // Esse vertor vai receber valores em String


    {Vertor.push("lll".to_string());} // Criando scopo para alocar o valor no Vertor 


    // Criando um verto ja definido 

    let mut vertorDefinido: Vec<String> = vec![
        String::from("Lanby"), 
        String::from("D#code.ex3"),
        String::from("Mordare")
    ];

    {  
        let mut a: i128 = 10;
        while a != 0 {

            vertorDefinido.push(format!("{}", a ).to_string());
            a -= 1;
        }
    }

    // Criei um Scopo para ver o funcionamento do  Vertor 

    println!("{:?}", vertorDefinido);


    // Mechendo com Com os vertores
     {
        vertorDefinido.insert(2, String::from("LAnby")); // Estou substituindo o valor da possição 2 para alocar o nome Lanby
        // Cli : ["Lanby", "D#code.ex3", "Mordare", "10", "9", "8", "7", "6", "5", "4", "3", "2", "1"]

        //["Lanby", "D#code.ex3", "LAnby", "Mordare", "10", "9", "8", "7", "6", "5", "4", "3", "2", "1"]
        

        vertorDefinido.remove(1); // Usando o valor na posição 1 vai remover 

        // cli : ["Lanby", "LAnby", "Mordare", "10", "9", "8", "7", "6", "5", "4", "3", "2", "1"]
     
     // Outra forma de mudar o valor 

     vertorDefinido[8] = "Lanbyteste".to_string();  // Tambem funciona 
     }

     println!("{:?}", vertorDefinido);


     // Lendo todo o vertor 

     for inten in vertorDefinido.iter() { // Tambem eu posso tira O ITER()
        println!(" Valor {}", inten);
     }

     println!("\x00l%00anb\x00y");
}