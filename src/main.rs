fn main(){
    //const values for yt dinks 
    const YT_PREPEND_URL : &str = "https://www.youtube.com/watch?v=";
    const YT_TIME_JOIN : &str = "&t=";
    const YT_TIME_S : &str = "s";
    //const values for libsyn dropin
//TODO    const POD_LINK_PREPEND : &str = " 

    let time1 :&str = "02:34:21";
    let time2 :&str = "30:01";
    let time3 :&str = "42.03";//with a full stop not semi should crash it with panic.
    println!("time1 in secs: {}", returns_time_in_seconds(time1));
    println!("time2 in secs: {}", returns_time_in_seconds(time2));
    println!("time3 in secs: {}", returns_time_in_seconds(time3));

}


fn returns_time_in_seconds (input : &str) -> i32 {
    let mut ret_num :i32 = 0;
    let mut semicolon_counter = 0;
    for i in input.chars() {
        if i == ':'{
            semicolon_counter+=1;
        }
    }
    if semicolon_counter == 1 {
        ret_num += input[0..2].parse::<i32>().unwrap()*60;
        ret_num += input[3..].parse::<i32>().unwrap();
    }
    else if semicolon_counter == 2 {
        ret_num += input[0..2].parse::<i32>().unwrap()*60*60;
        ret_num += input[3..5].parse::<i32>().unwrap()*60;
        ret_num += input[6..].parse::<i32>().unwrap();
    }
    else {
        panic!("This should be handled, in the case of not 1 or 2 semicolons.");
    }
    ret_num
}


