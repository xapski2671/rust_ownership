fn main() {
    let _s = "hello"; // s is valid from this point forward
    // do stuff with s

    let mut mem = String::from("hello");
    mem.push_str(", world");
    println!("{}", mem);

    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    fn calculate_length(s: &String) -> usize {
        s.len()
    }
} // this scope is now over, and s is no longer valid
