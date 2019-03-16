pub fn run() {
    let name = "Richard";
    let mut age = 37;
    println!("My name is {} and I am {}", name, age);
    age = 38;
    println!("My name is {} and I am {}", name, age);

    // Define constant
    const ID: i32 = 1;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Richard", 37);
    println!("{} is {}", my_name, my_age);
}
