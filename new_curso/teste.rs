

#[derive(Debug)]
enum Teste{
    Nome {
        Nome: String
    },
    Sobrenome {
        Sobrenome: String
    }
}
fn main() {

    let x = Teste::Nome{

        Nome:String::from("Lanby "),
    };


    println!("{:#?}",x);

}