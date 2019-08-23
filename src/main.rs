#![allow(unused)]

struct Rectangle {
    length: u32,
    width: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn five() -> i32 {
    5
}

fn main() {


    let rect1 = Rectangle { length: 50, width: 30 };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    

  let mut y:i32 = five();

  println!("The value of y is: {}", y);
  
  y = 6;
  println!("The value of x is: {}", y);
  
  let guess: u32 = "42".parse().expect("Not a number!");
  let c = 'z';
  let z = 'ℤ';
  
  let heart_eyed_cat = '😻';
  
  println!("The value of heart_eyed_cat is: {}", heart_eyed_cat);
  let x: (i32, f64, u8) = (500, 6.4, 1);
  println!("The print of tpules: {}, {}, {}", x.0, x.1, x.2);

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");


    for number in (1..4) {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
    
    
    //String type -> 
    let my_string = String::from("hello world");

    // first_word가 `String`의 슬라이스로 동작합니다.
    let word = first_word(&my_string[..]);
    
    //reference type &my_string 
    let word = first_word(&my_string);

    // error string type is not reference type &my_string
    //let word = first_word(my_string);


    let my_string_literal = "hello world";

    // first_word가 스트링 리터럴의 슬라이스로 동작합니다.
    let word = first_word(&my_string_literal[..]);

    // 스트링 리터럴은 *또한* 스트링 슬라이스이기 때문에,
    // 아래 코드도 슬라이스 문법 없이 동작합니다!
    // literal type - > &str
    let word = first_word(my_string_literal);
    
    

 
}   