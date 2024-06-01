pub mod error;

use structure::BigNum;

fn main() {
    let mut n1 = BigNum::from("923.4567");
    let n2 = BigNum::from("42123.6567");

    println!("1 = {:#?}", n1);
    println!("2 = {:#?}", n2);

    n1.add(&n2);
    println!("sum = {:#?}", n1);
}
