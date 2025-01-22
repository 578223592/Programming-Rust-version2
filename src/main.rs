use std::i64;
use std::str::FromStr;

fn add(x: i32, y: i32) -> i32 {
    x+y
}

// cargo run 123 456
fn main() {
    let mut vec = Vec::new();
    for x in std::env::args().skip(1) {
        let i = i64::from_str(&x).expect("from str err");
        vec.push(i);
    }

    for it in vec {
        println!("{}",it)
    }
}


#[test]
fn test_add(){
    assert_eq!(add(1,2),3);
    assert_eq!(add(1,2),4);
}