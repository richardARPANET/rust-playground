// Loops are used to iterate until a condition is met

pub fn run() {
    let mut count = 0;
    // Inf
    // loop {
    //     count += 1;
    //     println!("Number: {}", count);
    //     if count == 20 {
    //         break;
    //     }
    // }

    // While loop (fizzbuzz)
    // while count <= 100 {
    //     if count % 15 == 0 {
    //         println!("FizzBuzz");
    //     } else if count % 3 == 0 {
    //         println!("Fizz");
    //     } else if count % 5 == 0 {
    //         println!("Buzz");
    //     } else {
    //         println!("{}", count);
    //     }
    //     count += 1;
    // }

    // For range
    for count2 in 0..100 {
        if count2 % 15 == 0 {
            println!("FizzBuzz");
        } else if count2 % 3 == 0 {
            println!("Fizz");
        } else if count2 % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", count2);
        }
    }

}
