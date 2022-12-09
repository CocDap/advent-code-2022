
// what is input let vec = vec!["1000","2000","","4000","5000"]
// 
// logic : "" là dấu hiệu ngăn cách các elf với nhau  
// what is output : 9000
use std::fs::File;
use std::io::prelude::*;

fn main() {

    let mut file = File::open("./src/data/input.txt").unwrap();
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);
    //println!("{}", contents);
    let mut elves= vec![];
    for item in contents.trim().lines(){
        elves.push(item);
    
    }
    let mut order_elves = vec![];
    // println!("{:?}",elves);
    //let elves = vec!["1000","2000","8000","","9000","10000","","50000"];

    //let elves = vec!["5104", "6131", "3553", "4496", "5847", "3253", "1828", "1045", "6369", "5544", "2756", "7382", "", "6879", "9715", "10122"];
    let mut max =0 ;
    let mut temp = 0;
    for (i,item) in elves.iter().enumerate() {
        match item.parse::<i32>() {
            Ok(value) => {
                println!("value:{}",value);
                temp = temp + value; 
                if i == elves.len()-1 {
                    order_elves.push(temp);
                    println!("max:{},temp:{}",max,temp);
                    if max < temp {
                        max = temp;
                        
                    }
                
                }   
            },
            Err(_) => {
                order_elves.push(temp);
                if max < temp {
                    max = temp;
                    

                    temp = 0;
                }

                else {
                    temp =0;
                    continue;
                }

                
                
            }
        
        
        }
    
    }


    println!("MaX:{}",max);
    order_elves.sort();
    println!("Order vec:{:?}", order_elves);
    println!("Sum:{:?}", order_elves[order_elves.len()-3..order_elves.len()].to_vec().iter().sum::<i32>());

}



