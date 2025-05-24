use std::env;
 fn main(){
    let word="nemo";

    let string = env::args()
        .skip(1)
        .reduce(|accum,item|accum+" "+&item)
        .expect("string is expected");
    let index=string
        .split(" ")
        .position(|item|item==word)
        .expect("the word nemo was not found");
    println!("Found the word at {} ",index);
 }