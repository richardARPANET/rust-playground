// Primitive str: Immutable, fixed len str somewhere in memory
// String = Growable, heap-allocated data structure - use when need to modify of own string data.

pub fn run() {
    // String::from == growable
    let mut hello = String::from("Hello ");

    // Get length
    println!("Length: {}", hello.len());

    // .push CHAR
    // hello.push('Wo'); // NOPE: char is single
    hello.push('W');
    hello.push_str("orld!");

    // Capacity in bytes
    println!("Capacity {}", hello.capacity());
    // Check if empty
    println!("Is empty: {}", hello.is_empty());
    // Check if contains substr
    println!("Contains 'World': {}", hello.contains("World"));
    // Replace
    println!("Replace: {}", hello.replace("World", "There"));
    // Loop through String by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }
    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("{}", s);

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", hello);
}
