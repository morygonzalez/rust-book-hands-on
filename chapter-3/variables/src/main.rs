fn main() {
    println!("mutation()");
    mutation();
    println!("shodowing()");
    shodowing();
}

fn mutation() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

fn shodowing() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
}
