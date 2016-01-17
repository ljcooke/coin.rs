extern crate rand;

fn main() {
    println!("{}", match rand::random() {
        true => "Heads",
        false => "Tails",
    });
}
