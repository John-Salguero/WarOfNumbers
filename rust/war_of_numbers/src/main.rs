use std::env;

fn args_to_vec(argv: Vec<String>) -> Vec<i32>{
    let mut i = 1;
    let mut vec:Vec<i32> = Vec::new();
    while i < argv.len(){
        vec.push(a_to_i(&argv[i]));
        i+=1;
    }

    return vec;
}

fn a_to_i(s_num:&String) -> i32 {
    let mut retVal: i32 = 0;

    for c in s_num.as_bytes() {
        retVal *= 10;
        if c >= &b'0' && c <= &b'9'{
            retVal += i32::from(c - b'0');
        } else {
            panic!("string is not purely a positive number");
        }
    }

    return retVal;
}

fn main() {
    let nums: Vec<i32> = args_to_vec(env::args().collect());
    let mut odd = 0;
    let mut even = 0;

    for num in nums {
        if num & 1 == 0 {
            even += num;
        } else {
            odd += num;
        }
    }

    println!("Even total:{}; Odd total:{}",even,odd);
    if odd > even{
        println!("Odd is greater");
        println!("Difference: {}", odd - even);
    } else {
        println!("Even is greater");
        println!("Difference: {}", odd - even);
    }
    
}
