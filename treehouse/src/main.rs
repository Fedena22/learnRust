use std::io::stdin;

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Faild to read line");
    your_name.trim().to_lowercase()
}
fn main() {
    println!("What is your name?");
    let name = what_is_your_name();
    let visitor_list = ["bert", "steve", "fred"];
    let mut allow_them_in = false;
    for vistior in &visitor_list {
        if vistior == &name {
            allow_them_in = true;
        }
    }
    if allow_them_in {
        println!("Hello {}, welcome to the treehouse", name);
    } else {
        println!("Nope, you are not on the list");
    }
}
