use std::mem;

fn main() {

    let mut str: String = String::from("I'm Tonway!");


    str.push(' ');
    str.push_str( "I can do it");

    println!("{}\r\n", str);


    ////// 
    let sparkle_heart = vec![240, 159, 146, 150, 222];

    let sparkle_heart = String::from_utf8(sparkle_heart);


    let str = match sparkle_heart {
       Ok(value) => value,
       Err(_)=> {
        //panic!("error {}", err);
        String::from("err\r\n")
       }
    };

    println!("{}", str);


    //////
    let s = ['h', 'e', 'l', 'l', 'o'];

    let size:usize = s.into_iter().map(|c| mem::size_of_val(&c)).sum();

    println!("size: {}", size);
    


    ////////
    let s = "hello";
    assert_eq!(s.len(), 5);

    let s = "💖💖💖💖💖"; //4 bytes;
    assert_eq!(s.len(), 20);


    let s = ['💖', '💖', '💖', '💖', '💖'];

    let size: usize = s.into_iter().map(|c| mem::size_of_val(&c)).sum();

    println!("the heart size: {}", size);



    //////
    let s = "hello";

    let third_character = s.chars().nth(2);

    match third_character {
        Some(v) => {
            println!("{}",v);
        },
        None => {
            println!("none");
        }
    }

    let s = "💖💖💖💖💖";
    let third_character = s.chars().nth(2);

    assert_eq!(third_character, Some('💖'));



    //////
    // The first byte is 104 - the byte value of `'h'`
    let s = "hello";
    assert_eq!(s.as_bytes()[0], 104);
    // or
    assert_eq!(s.as_bytes()[0], b'h');

    // The first byte is 240 which isn't obviously useful
    let s = "💖💖💖💖💖";
    assert_eq!(s.as_bytes()[0], 240);
    




    
}
