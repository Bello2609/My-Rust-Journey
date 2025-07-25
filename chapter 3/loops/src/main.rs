fn main() {
    let mut counter = 0;
    let result = loop {
       counter += 1; 
       if counter == 10 {
            break counter * 2;
       }
    };
    println!("The result is {}", result);
    while_loop();
    loop_arr();
    for_loop();
    rev_for_loop();
   
}
fn while_loop() {
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("Light Off");
}
fn loop_arr() {
    let a = [ 10, 20, 30, 40, 50 ];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index = index + 1;
    }
}
fn for_loop() {
    let a = [ 10, 20, 30, 40, 50 ];
    for element in a.iter() {
        println!("The value is: {}", element);
    }
}
fn rev_for_loop() {
    for number in (1..6).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}