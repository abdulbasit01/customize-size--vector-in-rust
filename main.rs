use std::io;
fn main(){
    sum()
}
fn sum(){
    let mut a = String::new();
    println!("How many number you want to add");
    io::stdin().read_line(&mut a);
    let _type1 : i32 =a.trim().parse().unwrap();
    let mut v: Vec<i32> = Vec::new();
    for i in 0.._type1{
        let mut a = String::new();
        println!("Enter your numnber");
        io::stdin().read_line(&mut a);
        let _type2 : i32 =a.trim().parse().unwrap();
        v.push(_type2);
    }
    println!("{:?}",v )
}