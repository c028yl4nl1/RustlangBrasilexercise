fn perguntar_maior(valor: &[i32]) -> i32 {
    let mut acresentar: i32 = valor[0];

    for &number in valor {
        if number > acresentar {
            acresentar = number;
        }
    }
    acresentar
}
/* 
fn Tamanho<T>(valor: &[T]) -> i32 {
    let valors: i32 = valor.len().try_into().unwrap();


    valors
}
*/

struct  TiposVeiculos<Lanby>{
    Moto:Lanby,
    Carro:Lanby,
    Caminhao:Lanby,

}


impl <Lanby> TiposVeiculos<Lanby> {

    fn new() -> 

}
fn main() {
    let arrayNumeros = [4,5,6,3,55,4545,5453,343];

    let value = perguntar_maior(&arrayNumeros);

    println!("{}", value);


    /// Testando o Tipo generico com o <Lanby>
    {
        let alocar_value = TiposVeiculos{
            Moto:10
            Carro: 10
            Caminhao: 10
        };


    }

}


// Esse codigo não funciona , Não aprendir muito bem 