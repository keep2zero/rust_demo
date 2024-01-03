 
use std::{fs::File, io::{Read, Write}};

fn main() {
    println!("Hello, world!");

   let mut file =  File::open("assets/hello.txt").unwrap_or_else(|err| {
      panic!("error: {}", err); 
   });
   let mut buf = String::new();
  
   loop {

     let  size = file.read_to_string(&mut buf).unwrap();
        
     println!("the read size: {}", size);
     if size <= 0 {break;}
   }



   /// write file to the local fold
   let mut file = File::create("assets/demo.txt").unwrap();

   let isok = file.write_all(String::from("hello, file!").as_bytes()).is_ok();

   println!("write file {}", isok);
   
   
}