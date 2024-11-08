// TODO: fix bug where import order is changed.
use package_a::module_b;
use package_b::module_one;

fn main() {
    module_one::say_hello();
    let ignore = module_b::ignore("Everyone");
    println!("{}", ignore);
}
