fn main() {

    let mut s = String::from("hello");

    // Append a string to the one just created
    s.push_str(", Jon I see you are learning Rust");

    println!("s={}", s);

    memory_clone();

    calculate_length(&s);

    change_s(&mut s);
}

/* The following code generates a move compile time error because s1 was moved to s2 and cannot then be refereced.
fn memory_error() {
    let s1 = String::from("Hello Jon!");
    let s2 = s1;
    println!("s1={}", s1);
}
*/

// This avoids the problem because it clones s1 which does not move it.
fn memory_clone() {
    let s1 = String::from("Hello Jon! this is an example of cloning a variable");
    let s2 = s1.clone();
    println!("s1={} s2={}", s1, s2);
}

// This passes a ref to a String and hence does not take owndership of the String
// The String s cannot be changed
fn calculate_length (s: &String) -> usize {
    let s_len = s.len();
    println!("s.len={}",s_len);
    s_len
}

// This passes a mutable ref to a String and hence enables the function to change the contents of s
fn change_s (s: &mut String) -> usize {
    s.push_str(", This text was added to the end of the original string");

    let s_len = s.len();
    println!("s.len={} s={}",s_len, s);
    s_len
}
