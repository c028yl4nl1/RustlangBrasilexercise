mod helps {
    use std::io::stdin;

    pub fn InputString() -> String { //! Uma função que retonar a String  digitada no Input 

        let mut nulo = String::new();

        match stdin().read_line(&mut nulo) {
            Ok(_) => nulo.trim().to_string(),
            Err(_) => String::from(" Erro ao ler ")
        }


    }


    
    pub fn Inputint() -> isize {
        //! Essa Função retonar um isize ele convert uma String para inteiro isize

        let mut nulo = String::new();
        

        match stdin().read_line(&mut nulo){
            Ok(_) => {

                let x = | nulo: String | -> isize {
                    let a = nulo.trim().parse::<isize>().unwrap();
                    a
                    
                }; 

                let nulo1 = x(nulo); 
                nulo1
            
            }, 
            Err(_) => {
                println!(" Erro ao ler ");
                panic!();
            }
        }


    }

    pub fn Soma(x:i128 , y:i128) -> i32 {
        (x + y) as i32
    }
    pub fn Multiplicar(x:i128, y: i128) -> i32 {
        (x * y) as i32
    }

        
    
}


fn main() {
    /* 
    let a = helps::InputString(); // o valor vai ficar aqui 
    let b = helps::InputString(); // o valor vai ficar aqui 
    let c = helps::InputString(); // o valor vai ficar aqui 

    let d = helps::InputString(); // o valor vai ficar aqui 
    let e = helps::InputString(); // o valor vai ficar aqui 

    let f = helps::InputString(); // o valor vai ficar aqui 

    let g = helps::InputString(); // o valor vai ficar aqui 
    */

    let h = helps::Soma(0xfff,0xfff);

    println!("{} {}", h, 0xfff );


    let i = helps::Multiplicar(0xfff, 0xfff);

    println!("{} {}", i, 0xfff);
}