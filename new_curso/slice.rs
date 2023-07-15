fn main() {


    let  frase = String::from(" Ola essa é minha string slice ");

    // let a = &frase[0..34].; = erro Porque passou do limite da String slice 
    let a = &frase[0..]; // Ate a ultima posição 

    let a = &frase[0..frase.len()]; 
    println!("String slice {:?} ", a);

    println!("{}", frase);



   

    

}

/* 

fn meu_teste(){

    let v = String::from("Ola esse é o meu teste ");

    let c = v.as_bytes(); // cria um vertor as_bytes 
    let mut form = String::new();
    for (a , &s ) in c.iter().enumerate(){

        println!("s {} a {}", &s, a);
        
        
    }

    println!("{:?}", c );

    println!("{}", form);

} */