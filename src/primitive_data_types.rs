// primitive datatypes in rust
// means orignal not derived from something else
// basics and fundamentals

fn main(){

// Arrays
// Always have defined data_type of its item,
// Number of elements must be defiend

let  numbers : [u8;6] = [1,2,3,4,5,6];
let strings : [&str; 3] =["Monkey","Apple","Cake"];
println!("Numbers:{:?}\n Strings:{:?} ",numbers,strings);

// tuples
// can containe hetrogenous data of any length and any datat_ype

let my_tuple  = ("Mohid",30,false,[1,2,3,4,5,6]);
println!("\nSlices : {:?}",my_tuple);

// if you need a dynanmic Array length then you can go with Vectors

let number: Vec<i32> = Vec::new();

// Slices
let number_slices: &[u8] = &[1,2,3,4,5,6,7];
 let  number_slices: &mut[u8] = &mut[1,2,3,4,5,6,7];
number_slices[0]=64;
 let mut numbers2: [u8; 10] = [8, 9, 10, 11, 12, 13, 14,15,19,13];
 let  number_slices: &mut[u8] = &mut numbers2;
 number_slices[0]=32;
// inorder to make it mutable we pass a mutable array
println!("{:?}",number_slices);
println!("{:?}",numbers2);

}