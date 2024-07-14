#![allow(dead_code)]
use std::fmt::Write;

// So the main entry point for rust is main function
fn main() {
    // printing_methods_rust();

    primitive_data_types_rust();
}

// functions can be decalred by using fn keywords

fn printing_methods_rust() {
    // Adds a newline at the end
    //  println!("Hello, world!");

    // Dosent adds a newline
    //  print!("Hello,world! \n");

    // Adding the variables in string
    // println!("Hi my name is '{}'" , "mohid");

    // let s = format!("Hello world form {} {}! ", "Rust" ,"Its Core");
    // println!("{}",s);

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
    //     write!(&mut s, "Number: {}\n", i).unwrap();
    // }
    // println!("{}", s);

    // also pass the formated string into that
    // let mut string_buffer = format!("This is a already generated string by {}\n" ,"Rust" );

    // for i in 0..5{
    //     write!(&mut string_buffer,"Number Index {}\n",i).unwrap()
    // }
    // println!("{}", string_buffer);
}

// primitive datatypes in rust
// means orignal not derived from something else
// basics and fundamentals



fn primitive_data_types_rust() {
    //  int,float,bool,char
    //  signed_integers
    //  i8,i16,i32,i64,i128
    //  unsigned_integers
    //  u8,u16,u32,u64,u128

    // Defining data types with type annotations

    // let b : i8 = 127;
    // let c : i32 = 12000;
    // let d : i64 = 12000000000;
    // let d : i128 = 120000000000000000000000000000000000000;

    // Signed integer types
    let a: i8 = i8::MAX;
    let b: i8 = i8::MIN;
    let c: i16 = i16::MAX;
    let d: i16 = i16::MIN;
    let e: i32 = i32::MAX;
    let f: i32 = i32::MIN;
    let g: i64 = i64::MAX;
    let h: i64 = i64::MIN;
    let i: i128 = i128::MAX;
    let j: i128 = i128::MIN;
    let k: isize = isize::MAX;
    let l: isize = isize::MIN;

    // Unsigned integer types
    let m: u8 = u8::MAX;
    let n: u16 = u16::MAX;
    let o: u32 = u32::MAX;
    let p: u64 = u64::MAX;
    let q: u128 = u128::MAX;
    let r: usize = usize::MAX;

    let mut string_buffer = String::new();

    write!(&mut string_buffer, "Primitive data types in rust\n").unwrap();
    // Write signed integer values to buffer
    write!(&mut string_buffer, "Signed 8: max = {}, min = {}\n", a, b).unwrap();
    write!(&mut string_buffer, "Signed 16: max = {}, min = {}\n", c, d).unwrap();
    write!(&mut string_buffer, "Signed 32: max = {}, min = {}\n", e, f).unwrap();
    write!(&mut string_buffer, "Signed 64: max = {}, min = {}\n", g, h).unwrap();
    write!(&mut string_buffer, "Signed 128: max = {}, min = {}\n", i, j).unwrap();
    write!( &mut string_buffer,"Signed isize: max = {}, min = {}\n", k, l).unwrap();

    // Write unsigned integer values to buffer
    write!(&mut string_buffer, "Unsigned 8: max = {}\n", m).unwrap();
    write!(&mut string_buffer, "Unsigned 16: max = {}\n", n).unwrap();
    write!(&mut string_buffer, "Unsigned 32: max = {}\n", o).unwrap();
    write!(&mut string_buffer, "Unsigned 64: max = {}\n", p).unwrap();
    write!(&mut string_buffer, "Unsigned 128: max = {}\n", q).unwrap();
    write!(&mut string_buffer, "Unsigned usize: max = {}\n", r).unwrap();

    print!("{}", string_buffer);



}
