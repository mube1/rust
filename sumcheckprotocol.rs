//  in progress working on sumcheck protocol
// THIS IS INTENDED FOR FOLLOWING ALONG THE ALGORITHM

#[allow(dead_code)]
#[allow(unused_variables)]

//  a program that takes as input a polynomial `g` 
// (that the prover works with in each round, 
    // and the verifier only "consults" in the last round, 
    // and (either explicitly or implicitly) the number of variables `v`.

pub struct XVariable{         x1:i32, x2:i32, x3:i32,        }

struct Proover { }
struct Verifier {  p:Proover , round_values: Vec<i32>, }

impl Proover{
    fn s(&self,round:i32,x:i32)->i32{
        let r:i32 =0;
        if round==0{

        return polynomial_at_x(get_variable(x,0,1)) +polynomial_at_x(get_variable(x,1,0)) + polynomial_at_x(get_variable(x,0,1)) +polynomial_at_x(get_variable(x,1,0)) ;

        } 
        else if round==1{            
            return polynomial_at_x(get_variable(r,x,1)) +polynomial_at_x(get_variable(r,x,0)) ;
        }
        else if round == 3{
            return polynomial_at_x(get_variable(x,0,1)) +polynomial_at_x(get_variable(x,1,0)) ;
        }
        else{
            println!("No method found");
            return 0;
        }
    }
}

impl Verifier{
   let round_values=self.round_values;
   fn verify(&self,round:i32, value:i32)->bool{
        self.p.s(round,0)+self.p.s(round,1) == value
   }
}

fn get_variable(x1:i32,x2:i32,x3:i32)->XVariable{         XVariable{x1,x2,x3}    }

fn sumcheck_protocol(v:Verifier,round:i32)->bool{

    for i in 0..round{
        println!(" Round {} ", i);
        if v.verify(i,1) {
            v.round_values.push(v.p.s(i,0))
            continue;
        }
        else{
            return false;
        }
        
    }
    true

}


pub fn polynomial_at_x(x:XVariable)->i32 {
        2*(x.x1^3)+(x.x1*x.x3)+ (x.x2*x.x3)
}

fn main() {

        let round:i32= 3;
        let r: Vec<i32> = Vec::new();

        let p=Proover{};
        let v=Verifier{p,round_values:r};

        if sumcheck_protocol(v,round){
            println!("Accept");
        }
        else{
            println!("Reject");
        }
        
      

}
