

#[derive(Debug)]
struct Veiculo {
    modelo: String , 
    vendindo: JaestaVendindo,
}


#[derive(Debug)]
enum JaestaVendindo{
    Ford {

        vendindo1: bool
    }, 
    Fox {
        vendindo1: bool
    }

}

// Usando uma enum Option  


/* 
enum Option{

    some // some é quando Voce receber um dado 
    None // None é quando Voce receber um valor vazio 

}
*/

fn main(){ 
    let a2 = Veiculo{
        modelo:String::from(" Ford "), 
        vendindo: JaestaVendindo::Fox {
            vendindo1: true
        }
    };
    let a1 = Veiculo{
        modelo:String::from(" Ford a"), 
        vendindo: JaestaVendindo::Ford {
            vendindo1: false
        }
    };

    println!("{:#?}", a1);
    Exemplo2(); 



}


// Criando Outro Exemplo 
#[derive(Debug)]
struct Usersbc {
    ativo: bool ,
    nome: String,
    ip: TipoDeIP , // Não é obrigatorio o uso de virgula no final da struct mas na doc fala que é bom por 

}
#[derive(Debug)]
enum TipoDeIP{
    Tipoclassec {
        c: String
    }, 
    Tipoclasseb {
        b: String
    }
}

// impl o enum 

impl TipoDeIP{

    fn ver(&self) {

        println!("{:?}", self);
    }
}
fn Exemplo2() {

    let add_valor = Usersbc{
        ativo: true ,
        nome: String::from("Lanby"),
        ip: TipoDeIP::Tipoclassec{
            c: String::from(" Classe c ")
        }
    };

    add_valor.ip.ver(); 


}