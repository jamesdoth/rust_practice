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

// -----------------------------------------------------------------------------------

// // // Silence some warnings so they don't distract from the exercise.
// #![allow(unused_mut, unused_variables)]

// fn main() {
//     // Challenge: Write a function "add" that takes *references* to two integer arguments,
//     // dereferences them and adds them together, and returns the result.
//     //
//     println!("1 + 2 = {}, even via references", add(&1, &2));

//     // This fancy stuff either gets the first argument as a String, or prints
//     // usage and exits if an argument was not supplied to the program.
//     let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
//         println!("Please supply an argument to this program.");
//         std::process::exit(-1);
//     });

//     // 1. Write a function `inspect` that takes a reference to a String, returns nothing, but
//     // prints whether the contents of the String is plural or singular. Then uncomment and run this
//     // code with `cargo run apple` and `cargo run apples'.  Hint: use `.ends_with("s")` on the
//     // String reference
//     //
//     inspect(&arg);

//     // 2. Write a function `change` that takes a *mutable* reference to a String and adds an "s" to
//     // the String if it doesn't already end with "s". Then uncomment and run the code below with
//     // `cargo run apple`.  Hint: use `.push_str("s")` on the mutable String reference to add an "s".
//     //
//     change(&mut arg);
//     println!("I have many {}", arg);

//     // 3. Write a function `eat` that accepts ownership of (consumes) a String and returns a bool
//     // indicating whether or not the String both starts with a "b" AND contains an "a".
//     // Hint 1: use `.starts_with("b")` and `.contains("a")`
//     // Hint 2: `&&` is the boolean "AND" operator
//     // Try running this program with "boat", "banana", and "grapes" as the arguments :-)

//     if eat(arg) {
//         println!("Might be bananas");
//     } else {
//         println!("Not bananas");
//     }
// }

// fn inspect(s: &String) {
//     if s.ends_with("s") {
//         println!("{} is plural", s)
//     } else {
//         println!("{} is singular", s)
//     }
// }

// fn change(s: &mut String) {
//     if !s.ends_with("s") {
//         s.push_str("s")
//     }
// }

// fn eat(s: String) -> bool {
//     // if s.starts_with("b") && s.contains("a") {
//     //     true
//     // } else {
//     //     false
//     // }
//     s.starts_with("b") && s.contains("a")
// }

// fn add(n1: &i32, n2: &i32) -> i32 {
//     n1 + n2
// }
// ___________________________________________________________________________________________________
// // 1. Define a trait named `Bite`
// //
// // Define a single required method, `fn bite(self: &mut Self)`.  We will call this method when we
// // want to bite something.  Once this trait is defined, you should be able to run the program with
// // `cargo run` without any errors.
// //
// trait Bite {
//     fn bite(self: &mut Self);
// }

// // 2. Now create a struct named Grapes with a field that tracks how many grapes are left.  If you
// // need a hint, look at how it was done for Carrot at the bottom of this file (you should probably
// // use a different field, though).
// //
// #[derive(Debug)] // include this line right before your struct definition
// struct Grapes {
//     rem_grapes: i32,
// }

// // 3. Implement Bite for Grapes.  When you bite a Grapes, subtract 1 from how many grapes are left.
// // If you need a hint, look at how it was done for Carrot at the bottom of this file.
// //
// // impl Bite for...

// impl Bite for Grapes {
//     fn bite(self: &mut Self) {
//         self.rem_grapes -= 1
//     }
// }

// fn main() {
//     // Once you finish #1 above, this part should work.
//     let mut carrot = Carrot {
//         percent_left: 100.0,
//     };
//     carrot.bite();
//     println!("I take a bite: {:?}", carrot);

//     // 4. Uncomment and adjust the code below to match how you defined your
//     // Grapes struct.
//     //
//     let mut grapes = Grapes { rem_grapes: 100 };
//     grapes.bite();
//     println!("Eat a grape: {:?}", grapes);

//     // Challenge: Uncomment the code below. Create a generic `bunny_nibbles`
//     // function that:
//     // - takes a mutable reference to any type that implements Bite
//     // - calls `.bite()` several times
//     // Hint: Define the generic type between the function name and open paren:
//     //       fn function_name<T: Bite>(...)
//     //
//     bunny_nibbles(&mut carrot);
//     println!("Bunny nibbles for awhile: {:?}", carrot);

//     fn bunny_nibbles<T: Bite>(vege: &mut T) {
//         vege.bite();
//         vege.bite();
//     }
// }

// #[derive(Debug)] // This enables using the debugging format string "{:?}"

// struct Carrot {
//     percent_left: f32,
// }

// impl Bite for Carrot {
//     fn bite(self: &mut Self) {
//         // Eat 20% of the remaining carrot. It may take awhile to eat it all...
//         self.percent_left *= 0.8;
//     }
// }
