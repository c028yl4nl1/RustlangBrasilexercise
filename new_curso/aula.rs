mod Helps {
    use std::io::stdin;
    pub fn InputHeap() -> String {
        let y: String = {
            let mut buffer = String::new();

            stdin().read_line(&mut buffer).expect(" BUffer 0xfff Error ");
            buffer
        };
        y
    }

    pub fn InputIsize() -> isize {
        let x  =  {
            let mut buffer = String::new();
            stdin().read_line(&mut buffer).expect(" Erro ao ler o buffer ");
            let Xy = buffer.trim().parse::<isize>();

            match Xy {
                Ok (_) => Xy, 
                Err (_) => {
                    println!("Apenas Number ");
                    panic!()}, 
            }
        }; 
        x.unwrap() // Funcinou 
    }
    pub fn Somar(x: isize , y: isize) -> i32 {
        //! Tem que passar dois argumentos 
        
        (x + y ) as i32 
    }
    pub fn editString_str(valor: &mut String , value: &str) { // Lembre do Ownerhip 
        *valor = value.to_string();  // tinha esquecido eu não quero retonar 
    }
    pub fn editIsize(valor: &mut isize, value: isize){                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          
        *valor = value; 
    }

    pub fn clear() {
        print!("\x1B[2J\x1B[1;1H");
    }

    pub fn Par_Or_Impar(x: isize) -> String {
        if x % 2 == 0 {
            String::from("Par")
        }
        else {
            String::from("Impar")
        }

    }

}


use crate::Helps::*;


fn main(){
/* 
    println!("Vamos usar a nossa lib de testes");

    let mut teste = String::from("Teste");
    

    println!("{:?}", teste );

    let mut Number = 0;

    editIsize(&mut Number);

    println!("{}", Number); 

*/

    let mut idade = 10;
    editIsize(&mut idade, 30 as i32 as isize);

    println!("{}", idade);

    clear();

    println!("{}",Par_Or_Impar(3));


    



}