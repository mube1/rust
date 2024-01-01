
fn evaluate_term(term:String,_x:Vec<i32>)->i32{

    let mut result : i32= 1;
    let mut cond=false;
    let mut counter=0;
    for ch in term.chars(){
    counter+=1;

        if cond{            cond=false;         result*=_x[(ch as i32-48) as usize-1 ] ;            continue;
        }
        
        if  ch as i32 >=48 && ch as i32 <=58{          result*=ch as i32-48 ;        }
        else{    
            if ch=='/'{
                return result/evaluate_term(term.to_string()[counter+1..term.len()-1].to_string(),_x);
            }
            if  ch != '*' {cond=true;}
            
        }
        
    }

    result
    
}


fn main(){
      let args: Vec<String> = env::args().collect();
  let term :i32 = args[1].parse::<i32>().unwrap();
    println!("{}",evaluate_term(term.to_string(),[2,2,1].to_vec()));
}

