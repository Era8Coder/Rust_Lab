/*
    How to access the i'th element in the vector
    And if we can't access it then how to use it?
*/

fn main(){
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");
    
    for i in &v {
        println!("{i}");
    }

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let ninth: Option<&i32> = v.get(9);
    match ninth {
        Some(ninth) => println!("The ninth element is {ninth}"),
        None => println!("There is no ninth element"),
    }

    let forth: &i32 = &v[3];
    println!("The forth element is {forth}");
}
