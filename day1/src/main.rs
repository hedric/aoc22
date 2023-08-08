//use std::fs;
fn main(){
    println!("AoC 2022, day1");

    // let s = fs::read_to_string("input.txt")
    // .expect("Should have been able to read the file!");

    let s = String::from("1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000");

    let lines: Vec <_> = s.lines().map(String::from).collect();
    
    println!{"{:?}",lines}
}
