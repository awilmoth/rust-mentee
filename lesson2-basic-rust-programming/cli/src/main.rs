use rand::Rng;
use std::env;

fn main() {
    let rand_num: u8 = rand::thread_rng().gen_range(1..20);

    let args: Vec<String> = env::args().collect();
    let number: u8 = args[1].parse().unwrap();

    match number==rand_num {
        true => println!("WIN!!"),
        _ => println!("LOSE!!")
    }

    println!("{}", number);
    println!("{}", rand_num);


}
