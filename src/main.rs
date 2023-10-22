use std::env;


struct StartingLetter {
    starts_with : String, 
    count : i32,
    words : Vec<String>
}

fn main() {
    let letters : Vec<&str> = ["a","b","c","d"].to_vec();
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
        println!("v in upper: {} and lower: {}",v[0..1].to_uppercase(), v[0..1].to_lowercase());
        if &v[..1].to_lowercase() == letters[0] {println!("First letter of {} matches with {}", v, letters[0])};
    }

    println!("full string is {}", together);
    
}
