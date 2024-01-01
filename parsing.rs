use std::env;

fn multiplication_term_evaluate(term:String,_x:Vec<i32>)->i32{

    let mut result : i32= 1;
    let mut cond=false;
    let mut index:usize=0;
    for ch in term.chars(){

        if cond{
            cond=false;
            index=(ch as i32-48) as usize;
            
            result*=_x[index-1 ] ;
            continue;
        }
        
        if  ch as i32 >=48 && ch as i32 <=58{
            result*=ch as i32-48 ;
        }
        else{
            
            if  ch != '*' {cond=true;}
            
            
        }
        
    }

    result
    
}
fn main(){

    let polynomial :String = args[1];
    // let polynomial = "2*x1*x2*x3*3".to_string();

    println!("{}",multiplication_term_evaluate(polynomial,[1,3,1].to_vec()));

}
