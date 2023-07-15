fn main(){

    println!(" Hoje vamos falar sobre Strings");

    let nome = "LANBY String Slice";


    let mut Palavra = String::new();

    Palavra.push('L');
    Palavra.push('A');
    Palavra.push('N');
    Palavra.push('B');
    Palavra.push('Y');

    println!("Palavra : {} Letras: {}", Palavra, Palavra.len());
    

 { 
    let mut a: Vec<isize> = Vec::new();
    for Palavras in nome.bytes() {

        println!("{}", Palavras);

        a.push(Palavras.into());
    }

    println!("{:?}", &a[0..a.len()]);

}

{

    let fatia = "Lanby"; 

    let a = &fatia[0..fatia.len()];

    println!("{}", a);
}
}