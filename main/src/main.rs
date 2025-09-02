use std::time::{Instant};
use num_bigint::BigUint;
use num_traits::One;

fn main() {

    let start = Instant::now();

    let mut num1: BigUint = BigUint::ZERO;
    let mut num2: BigUint = BigUint::one();

    let mut count = 2;

    while start.elapsed().as_secs() < 1 {
        
        let num3 = num1 + &num2;
        num1 = num2;
        num2 = num3;

        count += 1;

    }

    print!("VAL: {num2}     INDEX: {count}");
    
}
