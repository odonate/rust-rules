extern crate garden;
use garden::patch::vegetables::Carrots;
use garden::greenhouse::vegetables::Tomatoes;

fn main() {
    let a = Carrots {};
    let t = Tomatoes {};
    println!("I'm growing {:?} and {:?}!", a, t);
}
