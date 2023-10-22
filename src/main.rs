use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
//    dbg!(args);
    let mut together : String = String::new();
    for (i, v) in args.iter().enumerate() {
        if i == 0 {
        continue;
        }
        if i+1 > args.len(){
        together.push_str(v);
        }
        together.push_str(v);
        together.push_str(" ");
    }

    println!("full string is {}", together);
}
