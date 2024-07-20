#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use std::collections::HashMap;
use std::fmt::Write;
use std::time::Instant;

use num_bigint::BigInt;
use num_traits::{One, Zero};

// So the main entry point for rust is main function
// functions can be decalred by using fn keywords and take arguments which must have defined type:annotaions
// and can be called in main

// fn main() {

//  print('Hello World')

// }

// print to million

// Serveral ways of prinitng in Rust
// fn  main() {
// Adds a newline at the end
//  println!("Hello, world!");
//  println!("Hello, world!");

// Dosent adds a newline
//  print!("Hello,world! ");

// Adding the variables in string
// let name: &str = "MOHID";
// let age = 12;

// println!("Hi my name is {} my age is {}" , "MOHID" , 12);

// let welcome_note = format!("Hello world form {} {}! ", "Rust" ,"Its Core");

// after 400 lines i executed the below code

// println!("{}",welcome_note);

// Another Example use case of format String

// let table = "users";
// let query = format!("SELECT * FROM {} WHERE id = {}", table, 42);

// prints the standard Error wit new line
// eprintln!("Error: Something went wrong");

// Dosent adds a ne line
//  eprint!("Error: Something went wrong");

// sometimes you will need to debug code so rust provides  dbg!(x); macro
// that will log the file name and number you are on
// let x: i32 = 5;
// dbg!(x);

// this scenario can be useful when you need to create string incrementally
// this write! macro adds the string in buffer and you can print it latter

// let mut s = String::new();
// for i in 1..=5 {
//  write!(&mut s, "Number: {}\n", i).unwrap();
// println!("Number {i}");

// }

// println!("{}", s);

// also pass the formated string into that
// let mut string_buffer = format!("This is a already generated string by {}\n" ,"Rust" );

// for i in 0..5{
//     write!(&mut string_buffer,"Number Index {}\n",i).unwrap()
// }
// println!("{}", string_buffer);
// }

// A function that runs to 100000000 numbers and perform operations and before that we take the start time and end time of script

// fn main() {
//     let start = Instant::now();

//     let mut sum: i64 = 0;
//     for i in 0..1000000000 {
//         let a = i * i;
//         sum += a % 1000;
//     }

//     let duration = start.elapsed();
//     let millis = duration.as_secs() * 1000 + duration.subsec_millis() as u64;
//     println!("Rust loop took: {} milliseconds", millis);
//     println!("Sum: {}", sum); // Just to use the variable and prevent optimization
// }

// types of controll flow in rust

// fn main (){

// if number % 2 == 0 {
//         println!("{number} is a even Number");
//     }

// if condition {
//         println!("{number} number is not a prime number")
//     } else {

// }

// if condition {

// } else if condition {

// } else {

// }

// }

// PRIME NUMBER CALCULATOR
// fn main() {
//     let number = 30;

//     if number % 2 == 0 {
//         println!("{number} is a even Number");
//     }
//     if number % 3 == 0 {
//         println!("{number} number is mutiple of 3");
//     }
//     if number % 5 == 0 {
//         println!("{number} number is mutiple of 5");
//     }

//     if number <= 1 {
//         println!("{number} number is not a prime number")
//     } else {
//         let mut is_prime: bool = true;
//         for i in (3..number).step_by(2) {
//             println!("{} / {} = {}", number, i, number % i);
//             if number % i == 0 {
//                 is_prime = false;
//                 break;
//             }
//         }
//         if is_prime {
//             println!("{number} number is a prime number")
//         } else {
//             println!("{number} number is not a prime number")
//         }
//     }
// }

// function for calculating the elast common factor

fn main() {
    // let x  = calculate_even_numbers(2);
    // let x  = calculate_odd_numbers(1000);
    // let x  = calculate_finacci_numbers(180);
    // println!("{:?}",x);
    // println!("{}",x.len())
    let x = calculate_finacci_numbers_position(2);
    println!("{}", x)
}

fn calculate_even_numbers(limit: i32) -> Vec<i32> {
    let mut x: Vec<i32> = Vec::new();

    // for i in (2..=limit).step_by(2) {

    //     x.push(i)
    // }
    let mut i = 2;
    while i <= limit {
        x.push(i);
        i += 2;
    }

    return x;
}

fn calculate_odd_numbers(limit: i32) -> Vec<i32> {
    let mut x: Vec<i32> = Vec::new();
    let mut i = 1;
    while i <= limit {
        x.push(i);
        i += 2;
    }
    return x;
}

// fn calculate_finacci_numbers(limit:i32) ->Vec<BigInt>{
//     let mut x : Vec<BigInt> = Vec::new();
//     let mut previous_number: BigInt = BigInt::zero();
//     let mut next_number: BigInt =  BigInt::one();
//     x.push(previous_number.clone());
//     x.push(next_number.clone());
//     for i in 1..limit-1{
//         let new_number: BigInt = previous_number.clone() +next_number.clone();
//         x.push(new_number.clone());
//         previous_number = next_number;
//         next_number = new_number;
//     }

//     return  x;
// }

fn calculate_finacci_numbers(limit: i32) -> Vec<i128> {
    let mut x: Vec<i128> = Vec::new();
    let mut previous_number: i128 = i128::zero();
    let mut next_number: i128 = i128::one();
    x.push(previous_number);
    x.push(next_number);
    for i in 1..limit - 1 {
        let new_number: i128 = previous_number + next_number;
        x.push(new_number);
        previous_number = next_number;
        next_number = new_number;
    }

    return x;
}

fn calculate_finacci_numbers_position(position: i32) -> i128 {
    let mut previous_number: i128 = i128::zero();
    let mut next_number: i128 = i128::one();
    let mut x: i128 = 0;
    // if position == 0 {
    //     return 1;
    // }

    for i in 2..=position {
        let new_number: i128 = previous_number + next_number;
        previous_number = next_number;
        next_number = new_number;
        x = next_number;
        println!("INDEX {} ,NUMBER {}", i, x);
    }

    return x;
}
