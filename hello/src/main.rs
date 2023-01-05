use hello::{good_bye, greetings};
use rand::prelude::*;

fn main() {
    greetings();
    good_bye();

    let m: u32 = random();

    println!("int: {}", m);
}
