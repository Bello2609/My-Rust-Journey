fn main() {
    // this is just a comment
    let x = another(45);
    println!("The value of x is: {}", x);
  
}


fn another(x: i32) -> i32 {
    x + 1
}
