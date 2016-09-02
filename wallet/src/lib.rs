extern crate rand;

pub fn flip_coin() -> &'static str {
    match rand::random() {
        true => "Heads",
        false => "Tails",
    }
}
