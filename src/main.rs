fn main() {
    let count:i32 = 100;

    fizzbuzz(count);
}

fn fizzbuzz(italate_counter: i32){    
    for italater in 0..italate_counter {
        if italater % 3 == 0 && italater % 5 == 0 {
            println!("{} fizzbuzz",italater);
        }
        else if italater % 3 == 0 {
            println!("{} fizz",italater);
        }
        else if italater % 5 == 0 {
            println!("{} buzz",italater);
        }
        else{
            println!("{}",italater);
        }
    }
}
