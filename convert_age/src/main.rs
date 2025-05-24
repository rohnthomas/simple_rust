use std::io;
fn main(){

    loop{

    println!("enter your age broo");
    let mut age=String::new();

    io::stdin()
        .read_line(&mut age)
        .expect("Failed to get the age");

    let age:u32=match age.trim().parse(){
        Ok(num)=>num,
        Err(_)=>continue,
    };
    
    let age:u32=&age*365;
    println!("you are now almost {} days old",age);


    }

}