use crate::garden::vegetables::Asparagus;
use crate::garden::vegetables::Strawberry;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);

    let strawberry = Strawberry {};
    println!("I'm growing {:?}!", strawberry);
}
