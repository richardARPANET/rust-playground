/*
Primitive types
Ints unsigned (no negative) and signed:
    u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits in memory)

Floats: f32, f64
Boolean (bool)
Characters (char) (1 character)
Tuples
Arrays (fixed length)
 */

// Rust is statically types so must konw all the types of the variables
// at compile time. However, the compiler can usually infer the type we want
// to use based on the value and how we're using it.

pub fn run() {
    // Default is i32
    let x = 1;
    // Default f64
    let y = 2.5;
    // Add explicit type
    let z: i64 = 5454454645645645;
    // Find max size
    println!("Max i32 {}", std::i32::MAX);
    println!("Max i64 {}", std::i64::MAX);
    // Boolean
    let is_active: bool = true;
    let is_greater = 10 > 5;
    // Char
    let a1 = 'a';
    let face = '\u{1F600}';
    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}
