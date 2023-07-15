fn main() {

    let a: f32 = 1.4323232; // f32 float
    let b : f64 = 233332.67; // f64 floot 



    println!(" {} ", a as f64 + 4 as f64 + b  );
    // usando o "as" eu estou forçando ser o tipo especificado na frente

    loop {
        for c in 1..1000000 {
            let a = c as f64 ;
            let b = c as f64;


            teste(a,b);

        }
    }


}

fn teste(a: f64, b: f64){
    loop {

        a + 33232.3 + b;

    }
}