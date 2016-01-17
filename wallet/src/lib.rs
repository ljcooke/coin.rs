extern crate rand;

pub fn flip_coin<'a>() -> &'a str {
    match rand::random() {
        true => "Heads",
        false => "Tails",
    }
}
