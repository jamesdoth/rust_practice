// use practice::{ding, on_off, print_array, print_difference, print_distance};
// // use practice::*; <-- i think it brings over everything from lib.rs

// fn main() {
//     let coords: (f32, f32) = (6.3, 15.0);
//     print_difference(coords.0, coords.1);

//     let coords_arr: [f32; 2] = [coords.0, coords.1];
//     print_array(coords_arr);

//     let series: [i32; 7] = [1, 1, 2, 3, 5, 8, 13];
//     ding(series[6]);

//     let mess: ([i32; 2], f64, [(bool, i32); 2], i32, &str) =
//         ([3, 2], 3.14, [(false, -3), (true, -100)], 5, "candy");
//     on_off(mess.2[1].0);

//     print_distance(coords);
// }
//-----------------------------------------------------------------------------

// fn main() {
//     // This collects any command-line arguments into a vector of Strings.
//     // For example:
//     //
//     //     cargo run apple banana
//     //
//     // ...produces the equivalent of
//     //
//     //     vec!["apple".to_string(), "banana".to_string()]
//     let args: Vec<String> = std::env::args().skip(1).collect();

//     // This consumes the `args` vector to iterate through each String
//     for arg in args {
//         // 1a. Your task: handle the command-line arguments!
//         //
//         // - If arg is "sum", then call the sum() function
//         // - If arg is "double", then call the double() function
//         // - If arg is anything else, then call the count() function, passing "arg" to it.
//         if arg == "sum" {
//             sum()
//         } else if arg == "double" {
//             double()
//         } else {
//             count(arg)
//         };

//         // 1b. Now try passing "sum", "double" and "bananas" to the program by adding your argument
//         // after "cargo run".  For example "cargo run sum"
//     }
// }

// fn sum() {
//     let mut sum = 0;
//     // 2. Use a "for loop" to iterate through integers from 7 to 23 *inclusive* using a range
//     // and add them all together (increment the `sum` variable).  Hint: You should get 255
//     // Run it with `cargo run sum`
//     for i in 7..=23 {
//         sum += i
//     }

//     println!("The sum is {}", sum);
// }

// fn double() {
//     let mut count = 0;
//     let mut x = 1;
//     // 3. Use a "while loop" to count how many times you can double the value of `x` (multiply `x`
//     // by 2) until `x` is larger than 500.  Increment `count` each time through the loop. Run it
//     // with `cargo run double`  Hint: The answer is 9 times.
//     while x < 500 {
//         x *= 2;
//         count += 1
//     }

//     println!(
//         "You can double x {} times until x is larger than 500",
//         count
//     );
// }

// fn count(_arg: String) {
//     // Challenge: Use an unconditional loop (`loop`) to print `arg` 8 times, and then break.
//     // You will need to count your loops, somehow.  Run it with `cargo run bananas`
//     //
//     // print!("{} ", arg); // Execute this line 8 times, and then break. `print!` doesn't add a newline.
//     let mut count = 0;
//     loop {
//         println!("arg");
//         count += 1;
//         if count > 7 {
//             break;
//         };
//     }

//     println!(); // This will output just a newline at the end for cleanliness.
// }
