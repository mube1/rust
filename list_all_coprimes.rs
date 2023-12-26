use std::env;
use std::cmp;
//  This script receives a number and prints out the co primes less than it



fn gcd(num_1:i32,num_2:i32)-> i32{
    let mut a : i32 = cmp::max(num_1,num_2);
    let mut b : i32 = cmp::min(num_2,num_1);
    
    loop {       
        
        if a%b == 0{            break;        }
        (a,b)=(b,a%b);                              
    }
    b
}
fn list_of_relativeley_primes(number:i32){
    for i in (2..number){
        if gcd(number,i)==1{
            println!("{} ",i);
        }
    }
}

fn main(){
  let args: Vec<String> = env::args().collect();
  let number :i32 = args[1].parse::<i32>().unwrap();
  list_of_relativeley_primes(number);
}
