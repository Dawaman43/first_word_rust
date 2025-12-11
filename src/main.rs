fn main() {
    let my_s = String::from("Hello world");
  let f_s =   first_word(&my_s);
  println!("The first word is: {}", f_s);

}

fn first_word(s: &str)-> &str{
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
        
    }
    &s[..]
}