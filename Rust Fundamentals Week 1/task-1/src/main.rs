fn concatenate_strings(str1: &mut String, str2: &String) -> String {
 str1.push_str(str2);
 let result: String = str1.to_string();
 result
}
fn main() {
    let  mut string1: String = "Hello".to_string();
    let  string2: String = " World".to_string();

    println!("{}",concatenate_strings(&mut string1, &string2));
}
