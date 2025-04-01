// use std::io; <- no need of including io since they are not used in the below program ->

fn main(){
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("{:?}", v);
}

/*
    To compile and run the program <-
    rustc vector.rs && ./vector
*/