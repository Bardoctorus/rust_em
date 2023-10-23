use std::env;


struct StartingLetter {
    starts_with : String, 
    count : i32,
    words : Vec<String>
}

fn main() {
    let letters : Vec<&str> = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"].to_vec();
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
//        println!("v in upper: {} and lower: {}",v[0..1].to_uppercase(), v[0..1].to_lowercase());
        for i in 0..letters.len(){
            if &v[..1].to_lowercase() == letters[i] {println!("First letter of {} matches with {}", v, letters[i])};
        }
        for i in 0..letters.len(){
        println!("letter {} is {}",i, letters[i]);
        
        }
    }

    println!("full string is {}", together);
    
}
