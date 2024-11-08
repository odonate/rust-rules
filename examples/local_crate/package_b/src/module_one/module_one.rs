use package_a::module_a;

pub fn say_hello() {
    let greeting = module_a::greet("World");
    println!("{}", greeting);
}
