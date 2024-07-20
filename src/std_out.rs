
Serveral ways of prinitng in Rust
fn  main() {
// Adds a newline at the end
 println!("Hello, world!");
 println!("Hello, world!");

// Dosent adds a newline
 print!("Hello,world! ");

// Adding the variables in string
let name: &str = "MOHID";
let age = 12;

println!("Hi my name is {} my age is {}" , "MOHID" , 12);

let welcome_note = format!("Hello world form {} {}! ", "Rust" ,"Its Core");

// after 400 lines i executed the below code

println!("{}",welcome_note);

// Another Example use case of format String

let table = "users";
let query = format!("SELECT * FROM {} WHERE id = {}", table, 42);

// prints the standard Error wit new line
eprintln!("Error: Something went wrong");

// Dosent adds a ne line
 eprint!("Error: Something went wrong");

// sometimes you will need to debug code so rust provides  dbg!(x); macro
// that will log the file name and number you are on
let x: i32 = 5;
dbg!(x);

// this scenario can be useful when you need to create string incrementally
// this write! macro adds the string in buffer and you can print it latter

let mut s = String::new();
for i in 1..=5 {
 write!(&mut s, "Number: {}\n", i).unwrap();
println!("Number {i}");

}

println!("{}", s);

// also pass the formated string into that
let mut string_buffer = format!("This is a already generated string by {}\n" ,"Rust" );

for i in 0..5{
    write!(&mut string_buffer,"Number Index {}\n",i).unwrap()
}
println!("{}", string_buffer);
}