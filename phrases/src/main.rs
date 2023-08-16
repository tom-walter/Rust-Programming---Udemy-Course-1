extern crate phrases;
use phrases::greetings::{english, french};


fn main(){
    println!("EN: {}, {}", english::hello(), english::goodbye());
    println!("FR: {}, {}", french::hello(), french::goodbye());
}