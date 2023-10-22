use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    let mut together : String = String::new();
    // Iterate over the args ignoring the first, adding a space unless it's the end
    for (i, v) in args.iter().enumerate() {
        if i == 0 {continue;}
        if i+1 == args.len(){
            println!("hit end of ting");
            together.push_str(v);
            break;
        };
        together.push_str(v);
        together.push_str(" ");
    }

    println!("full string is {}", together);
}
