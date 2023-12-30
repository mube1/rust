// Reed Solomon Finger Print

// How two parties can confirm that each have the same message



use rand::Rng;

fn get_n_bits(n:usize)->String{
        // Generate random binary strings of size n
    let mut rng = rand::thread_rng();  
     let mut s = String::from("");

    for _i in 0..n{    
            s.push_str(&rng.gen_range(0..2).to_string());  
        }
    s
}


struct Data{  
    bits:String,
    // a class to represent two data holders, A and B
}
pub trait Eval {
    // Evaluate function that applies for both parties
    fn  evaluation(&self, value:i32, n:usize)->i32;
}

pub trait Generate {
    //  Generate a random number from field of size n^n
    fn  generate(&self,n:usize)->i32;
}
impl Generate for Data {
    fn  generate(&self,n:usize)->i32{
        let mut rng = rand::thread_rng();  
        rng.gen_range(0..n as i32)
    }
}
// trait eval()
impl Eval for Data {
        fn evaluation(&self,value:i32, n:usize)->i32{
        let mut v:i32=0;
        for i in 0..n{
            v+=(value^(i as i32)) *self.bits.as_bytes()[i] as i32;
        }
        v
    } 
}

fn reed_solomon_finger_print(a:Data,b:Data,n:usize)->bool{
        // Reed solomon message checking 

        // I am not sure about the field F from which I picked an r from
    let r=a.generate(n^n)%n; 
        // Alice a, generates message
    let mut condition;
    if a.evaluation(r,n)==b.evaluation(r,n){
          println!("Correct");
          condition=true;
    }
    else{
          println!("Wrong");
          condition=false;
    }
    condition
}
fn main() {

    let n:usize=3; 

    let a=Data{bits:get_n_bits(n)};
    let b=Data{bits:get_n_bits(n)};
        // Two parties with n bit strings
    println!("{}",reed_solomon_finger_print(a,b,n));

    
    
}

