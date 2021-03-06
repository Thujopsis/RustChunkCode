fn main() {
    let count:i32 = 100;

    for italater in 0..count {
    //println!("{}",fizzbuzz(italater));
    println!("{}",pattern_fizzbuzz(italater));
    }
}

fn fizzbuzz(italate_counter: i32) -> String{    
    if italate_counter % 3 == 0 && italate_counter % 5 == 0 {
        "fizzbuzz".to_string()
    }
    else if italate_counter % 3 == 0 {
        "fizz".to_string()
    }
    else if italate_counter % 5 == 0 {
        "buzz".to_string()
    }
    else{
        italate_counter.to_string()
    }
}

fn pattern_fizzbuzz(italate_counter :i32) -> String{
    match (italate_counter % 3, italate_counter % 5) {
        (0,0) => "fizzbuzz".to_string(),
        (0,_) => "fizz".to_string(),
        (_,0) => "buzz".to_string(),
        (_,_) => italate_counter.to_string(),
    }
}

#[test]
fn can_test(){
    assert_eq!(1,1);
}

#[test]
fn fizzbuzz_divided3(){
    assert_eq!(fizzbuzz(3),"fizz".to_string());
}

#[test]
fn fizzbuzz_divided5(){
    assert_eq!(fizzbuzz(5),"buzz".to_string());
}

#[test]
fn fizzbuzz_divided15(){
    assert_eq!(fizzbuzz(15),"fizzbuzz".to_string());
}

#[test]
fn pattern_fizzbuzz_divided3(){
    assert_eq!(pattern_fizzbuzz(3),"fizz".to_string());
}

#[test]
fn pattern_fizzbuzz_divided5(){
    assert_eq!(pattern_fizzbuzz(5),"buzz".to_string());
}

#[test]
fn pattern_fizzbuzz_divided15(){
    assert_eq!(pattern_fizzbuzz(15),"fizzbuzz".to_string());
}