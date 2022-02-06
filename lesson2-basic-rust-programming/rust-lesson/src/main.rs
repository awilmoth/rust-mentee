mod my_lib;

use my_lib::get_random_number;

fn main() {
    let num = get_random_number();
    println!("Hello, world!: {}", num);
}