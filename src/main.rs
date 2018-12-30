fn main() {
    const MAX_VALUE : i32 = 30;
    for index in 1..=MAX_VALUE {
        println!("{} {}", index, fizzbuzz(index));
    }
}

// fizzbuzz return 対応した文字列を返す
fn fizzbuzz(index: i32) -> String {
    if index % 15 == 0 {
        return "Fizz Buzz".to_string();
    } else if index % 5 == 0 {
        return "Buzz".to_string();
    } else if index % 3 == 0 {
        return "Fizz".to_string();
    }else {
        return "".to_string();
    }
}
