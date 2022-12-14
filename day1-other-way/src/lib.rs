pub fn process_part1(input:&str) ->String {

    // Step1: Print input
    // println!("Input:{}",input);

    //Step2: Split
    // let split = input.split("\n").collect::<Vec<&str>>();
    // println!("Split:{:?}",split);


    // Step3: Use Map
    // Seperate by space
    // parse string into integer
    // find max
    let res = input.split("\n\n").map(|cal|{
        cal.lines().map(|item| item.parse::<u32>().unwrap()).sum::<u32>()
    }).max().unwrap();

    res.to_string()
}



pub fn process_part2(input:&str) ->String {

    // Step1: Use Map
    // Seperate by space
    // parse string into integer
    //convert into vec


    let mut res = input.split("\n\n").map(|cal|{
        cal.lines().map(|item| item.parse::<u32>().unwrap()).sum::<u32>()
    }).collect::<Vec<u32>>();

    println!("res:{:?}",res);

    //Step2: Ordering 

    res.sort_by(|a,b| b.cmp(a));

    println!("res:{:?}",res);

    // Step3: Find sum 3 
    let max:u32 = res.iter().take(3).sum();
    max.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn it_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "24000");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "45000");
    }
}