fn main() {
    let mut s = String::from("Hello world");

    let word = first_word(&s);

    println!("{}", s);

    clone_method();
    
}
 fn clone_method() {
    let s1 = String::from("Hello");
    let s2  = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
 } 

 fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len();
 }
