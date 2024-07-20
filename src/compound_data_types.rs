// Finding the prime factor of a large number un optimized 
// fn main() {
//     let mut factors: Vec<i32> = Vec::new();

//     let a: i32 = 8699;

//     let  limit_to_factor: i32 = (a as f64).sqrt().floor() as i32;

//     let mut number = a;

//     while number % 2 == 0 {
//         factors.push(2);
//         number = number / 2;
//     }
   
//     for i  in (3..=limit_to_factor).step_by(2){
//         while number % i == 0 {
//             factors.push(i);
//             number = number / i;
//         }
        
//     }

//     if number > 1 {
//         factors.push(number);
//     }



//     println!("{:?}",factors)


// }

// optimized Code for prime factorization 


fn main() {
    let mut factors: Vec<i32> = Vec::new();
    let mut factors_hash_map: HashMap<i32,u8> = HashMap::new();

    let a: i32 = 8597;
    let mut number = a;

    // Check for divisibility by 2 first
    while number % 2 == 0 {
        factors.push(2);
        let count = factors_hash_map.entry(2).or_insert(0);
        *count += 1;
        number /= 2;
    }

    // println!("AFTER PERFORMING FIRST PRIME NUMBER DIVISION, INITIAL VALUE OF NUMBER: {number}");

    let mut i = 3;

    // Check for odd factors starting from 3
    while i * i <= number {
        // println!("CURRENT VALUE OF NUMBER: {number}");
        // println!("SQUARE OF ITERATION {i}: {}", i * i);

        while number % i == 0 {
            factors.push(i);
            let count = factors_hash_map.entry(i).or_insert(0);
            *count += 1;
            number /= i; // Correct division
        }
        i += 2;
    }

    // If the remaining number is greater than 1, it must be a prime factor
    if number > 1 {
        factors.push(number);
        let count = factors_hash_map.entry(number).or_insert(0);
        *count += 1;
    }

    println!("{:?}", factors);

    // let mut formatted_factors = String::new();

    // for item in factors {

    //     write!(&mut formatted_factors, "{}\n", item).unwrap();
        
    // }

    // println!("{formatted_factors}")


    println!("{:?}",factors_hash_map)

}