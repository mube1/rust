// This is the main function.
use std::env;



fn primality_check(number:f32)-> bool {

    if number as i64 % 2 == 0{
        // If its even, done.
        return false;
    }

    let limit:i64=number.sqrt() as i64;
   
    
    for i in 3..limit{
        // checking below its sqrt suffice to show primality
        
        if number as i64 % i == 0{            
            false;
        }

    }
 
        
        true
}

fn main() {

let args: Vec<String> = env::args().collect();

// let number: i32= &args[1];
let number :i32 = args[1].parse::<i32>().unwrap();

println!("{} ",primality_check(number as f32) as bool);
// the 

}