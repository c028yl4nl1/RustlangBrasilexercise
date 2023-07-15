use std::env::args;
use std::fs;





fn main(){
    println!("Programinha de linha de Comando");
    let args:Vec<String> = args().collect(); // Ler os argumentos


    let Config_args = Config::new(&args);
    let ver = match Config_args {
        Ok(valor) => valor,
        Err(Error) => {
            use std::process;
            println!("Error : {}", Error);
            process::exit(1);
    }, 
    };

    //println!("{:?}", ver); // Ok() -> Config

    // filename , search 

    let filename = ver.Filename.clone(); 
    let search = ver.search.clone(); 

    let arquivo_open = match arquivo(ver) {
        Ok(valor) => Ok(valor) ,
        Err(Error) => Err(format!("Não encontrei um arquivo {} ", Error)),
    };

    let Conteudo_Stringh = arquivo_open.unwrap();
    let  vertor: Vec<String> = {

    let mut Strings_: Vec<String> = Vec::new();
    for lines in Conteudo_Stringh.lines(){
        if lines.to_lowercase().contains(&search.to_lowercase()){
            Strings_.push(lines.to_string());
        }
        
    }

    Strings_ 
}; 

    for u in vertor {

        println!("{}", u);
    }

  
    // Lendo o arequivo 
/* 
    let Conteudo_arq = fs::read_to_string(arq_filename); //read_to_string("Filename")
                              // Result <String , Err>                                         ~

    match Conteudo_arq{
        Ok(_) => println!("Abrindo arquivo ...{:?}", &Conteudo_arq.expect("Não li").trim()),
        Err(Error) => panic!("Erro ao abri o arquivo Error: {}",Error),
    }
*/

          
  /*
    let mut Conteudo = String::new();
    let a = File::open(args[1].as_str());
    let x = a.expect("ERrp").read_to_string(&mut Conteudo);
  
*/ 


}
#[derive(Debug)]
struct Config{
    Filename: String ,
    search: String, 
}

impl Config{
    fn new(vertor: &[String]) -> Result<Config , &'static str>{

        if vertor.len() < 3 {
            return Err("Argumentos inapropriado");
        }
        let search: String = vertor[1].clone();
        let Filename: String = vertor[2].clone();

        let config = Config{search , Filename};
        Ok(config)
    }
}

fn arquivo(valores: Config) -> Result<String, String>{

    let arquivo = fs::read_to_string(valores.Filename);

    let x: Result<String, String> = match arquivo {
        Ok(valor) => Ok(valor),
        Err(Error) => Err(format!("Arquivo Não existe ou Não tenho permisão: {} ", Error)),
    }; 
    x


}

