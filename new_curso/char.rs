fn main() {


    let a = 'A';

    println!("{:?
    }", a);

    let b = " 🦁 ";

    println!(" \u{01F981} {:?} ", b);

    let c = concat!('a', 1.2 , 8);


    println!("{:?}", c );

}