
#[allow(dead_code)]
#[allow(unused_variables)]
//  in progress working on sumcheck protocol
// THIS IS INTENDED FOR FOLLOWING ALONG THE ALGORITHM

//  a program that takes as input a polynomial `g` 
// (that the prover works with in each round, 
    // and the verifier only "consults" in the last round, 
    // and (either explicitly or implicitly) the number of variables `v`.

pub fn polynomial_at_x(x:Vec<i32>)->i32 {    x[0]*x[0]*x[0]*2 + x[0]*x[2]+x[1]*x[2]}
struct Proover { }
struct Verifier { p:Proover ,  r: Vec<i32>}
 
impl Proover{
    // fn s(&self,round:usize,x:i32, r: &[i32])->i32{
    fn s(&self,round:usize,x:i32,r:Vec<i32>)->i32{
        
       if round==0{
           return polynomial_at_x([x,0,0].to_vec()) +
            polynomial_at_x([x,0,1].to_vec()) +
             polynomial_at_x([x,1,0].to_vec()) +
              polynomial_at_x([x,1,1].to_vec());
           
       }
       else if round==1{
            return polynomial_at_x([r[round-1 ],x,0].to_vec()) + 
            polynomial_at_x([r[round-1 ],x,1].to_vec());
       }
       else if round ==2 {
            return polynomial_at_x([r[round-2 ],r[round-1 ],x].to_vec()) ;
       }
       else{       
            return polynomial_at_x([r[round-3 ],r[round-2],r[round-1 ]].to_vec()) ;
       }
       
    }
}

pub trait Verfication {
    fn verify(&self,round:usize)->bool;
}

impl Verfication for Verifier{
    fn verify(&self,round:usize)->bool{

        if round==0{
                return self.p.s(round,0,self.r.clone()) +self.p.s(round,1,self.r.clone())==12;
        }
        else{           
                return self.p.s(round,0,self.r.clone()) +self.p.s(round,1,self.r.clone())== self.p.s(round-1,self.r[round-1],self.r.clone()) ;
        }
        
      
    }
}

fn sumcheck_protocol(v:Verifier,round:usize)->bool{
    let _h:i32=12;
    
    for i in 0..(round){
                println!(" Sumcheck round {} ", i);
                if v.verify(i) {                 
                    println!("Passed at round  {} ", round);
                    continue;
                }
                else{
                    println!("Failed at round  {} ",i);
                    return false;
                }    
    }
    true

}

fn main() {

        let round: usize= 3;

        let p=Proover{};
        let v=Verifier{p:p,r:[2,3,6].to_vec()};
        

        if sumcheck_protocol(v,round){
            println!("Accept");
        }
        else{
            println!("Reject");
        }
        
        
        
}
