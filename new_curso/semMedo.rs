use std::thread;
fn main(){

    println!("Ola hoje vamos fazer uma threads"); 


    let mut nome = "LanbyShell";

    let closure = move | nome: &str | {
        println!("Tomei posse do nome : {}", nome);
    };
    closure(nome); 

    nome = "agora";
    println!("{}", nome);

    let object = thread::spawn(|| {
        println!("UMa Thread
        ");
    }); // o spawn cria um objeto


    // object.join().unwrap() vai falar assim: eu sou uma thread , primeiro eu segundo vc thread principal. Resumindo: vai iniciar primeiro do que a thread main()
    object;  

    {

        // vou criar outro teste de uma thread

        let ver_usuario = || -> i32 {
            let mut r: i32 = 0;
            for c in 0..5{
                r = c ;
            }
            r
        }; |

        let ver = thread::spawn(ver_usuario);
        println!("{:?}", ver.join().unwrap());

    } 


  
}