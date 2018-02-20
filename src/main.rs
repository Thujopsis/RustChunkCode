fn main() {
    let count:i32 = 100;

    for italater in 0..count {
    fizzbuzz(italater);
    }
}

fn fizzbuzz(italate_counter: i32){    
    if italate_counter % 3 == 0 && italate_counter % 5 == 0 {
        println!("{} fizzbuzz",italate_counter);
    }
    else if italate_counter % 3 == 0 {
        println!("{} fizz",italate_counter);
    }
    else if italate_counter % 5 == 0 {
        println!("{} buzz",italate_counter);
    }
    else{
        println!("{}",italate_counter);
    }
}

#[test]
fn can_test(){
    assert_eq!(1,1);
}
