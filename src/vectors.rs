// Vectors - resizeable arrays

use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];
    // Reassign a value
    numbers[2] = 20;
    // Add onto vector
    numbers.push(5);
    numbers.push(6);
    // Pop off last
    numbers.pop();
    println!("{:?}", numbers);
    // Get single val
    println!("Single value: {}", numbers[0]);
    // Get length
    println!("Vector length: {}", numbers.len());
    // Amount of memory, stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));
    // Get slice
    let slice: &[i32] = &numbers[0..3];
    println!("Slice: {:?}", slice);
    // Loop through values
    for x in numbers.iter() {
        println!("Number {}", x);
    }
    // Loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Numbers vec: {:?}", numbers);
}
