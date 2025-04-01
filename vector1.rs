fn main(){
    let mut v = vec![1,2,3];       // Creating a vector which contains the values         
    println!("{:?}", v);    
    v.push(4);
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    // Common printing type
    println!("{:?}", v);
    
    println!("Hey Man");
    // Printing each number in different line
    println!("{:#?}",v);        // For printing in a new fashion :))
}