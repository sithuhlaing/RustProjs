mod vars;

fn main() {
    // print();

    vars::run();
}

fn print(){
    println!("{0} is from {1} and {0} likes to {2}", "Bred", "Mass", "code:");

    println!("{name} likes to play {activity}", name="John", activity="Baseball");

    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    println!("{:?}", (12, true, "Hello"));
}
