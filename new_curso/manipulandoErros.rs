use std::fs::File; 
fn main(){

        // Panic! 

        // panic!("Errou feio ");


        // Fazendo um exemplo de erro abrindo um arquivo 

        {
       
            
            let arquivo = match File::open("slice.rs"){
                Ok(Buffer) => Buffer,
                Err(Error) => panic!("Erro ao abri ao Abri o arquivo "), 
            };

            println!("{:?}", arquivo);
        }

        {

            // Fazendo outro exemplo abrindo um arquivo 
            use std::io::Read; 
            use std::io; 
            fn ver_arq() -> Result<String , io::Error> {

                let mut err_or_String = String::new();

                File::open("slice.rs")?.read_to_string(&mut err_or_String)?;
                Ok(err_or_String)
            }

            println!("{:?}", ver_arq());


        } 

        {

            // Outro exemplo de Manipular erros 

            let number = "3ffef"; // Vou tentar converter para i32
            let Result = number.parse::<i32>(); // Tentando converter 


            println!("{:?}", Result.unwrap_err()); // unwrap_err() vai descapsular o erro 

            println!(" Lanby ");


            {
                // Forçando o erro com o expect 
                let valor: i32 = "1".parse().expect("Deu ruim");
                // fazendo caso dar erro 

                let valor:i32 = "30s".parse().unwrap_or(20); // Caso dar erro o valor padrão vai ser 20
                println!("{}", valor);
            }



        
        }
}