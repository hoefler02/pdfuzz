use rand::distributions::Alphanumeric;
use rand::seq::SliceRandom;
use clap::Parser;
use rand::Rng;


/*
A PDF is made up of 4 main elements:
1. The Version Header
2. The Body (Objects)
3. XREF Table
4. Trailer (Locations)
*/

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Output PDF File
    #[arg(short, long)]
    output: String
}

fn main() {
    let args = Args::parse();
    println!("header: {}", gen_header(None::<f32>));
    println!("{}", gen_float());
    println!("{}", gen_string(1000));
    println!("{}", gen_hex_string(1000));
    println!("{}", gen_dict(10));
    println!("{}", gen_array(10));
    println!("{}", gen_bytes(10));
}

fn rand_range(low: u32, high: u32) -> u32 {
    rand::thread_rng().gen_range(low..high)
}

/*
    PART 1: Header
    Gens a random PDF header
    %PDF-1.1 through %PDF-1.7 (valid) or wildcard
*/
fn gen_header(wildcard: Option<f32>) -> String {
    if wildcard.is_none() {
        let ver: u8 = rand_range(0, 8) as u8;
        format!("%PDF-1.{}", ver)
    } else {
	    format!("%PDF-{}", wildcard.unwrap())
    }
}

/*
    PART 2: Body
    We start by generating random PDF objects
    These 8 types of objects compose the body
        1. Boolean
        2. Number
        3. String
        4. Name
        5. Array
        6. Dictionary
        7. Stream
        8. Null
*/

// fn gen_body() -> String {
//     random range and generate objects
// }

fn gen_bool() -> String {
    rand::thread_rng().gen::<bool>().to_string()
}

fn gen_int(signed: bool) -> String {
    if signed {
        rand::thread_rng().gen::<i8>().to_string()
    } else {
        rand::thread_rng().gen::<u8>().to_string()
    }
}

fn gen_float() -> String {
    (rand::thread_rng().gen::<f32>() * 100.).to_string()
}

fn gen_num() -> String {
    let opt = rand_range(0, 3);
    if opt == 0 {
        gen_int(false)
    } else if opt == 1 {
        gen_int(true)
    } else {
        gen_float()
    }
}

fn gen_string(max: u32) -> String {
    if rand::random() {
        gen_raw_string(max)
    } else {
        gen_hex_string(max)
    }
}

fn rnd_str(alpha: &str, max: u32) -> String {
    let alpha: Vec<char> = alpha.chars().collect();
    let len = rand_range(0, max);
    let mut res = String::new();
    for _ in 0..len {
        res.push(*alpha.choose(&mut rand::thread_rng()).unwrap());
    }
    res
}

fn gen_raw_string(max: u32) -> String {
    let s = rnd_str("0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ", max);
    format!("({})", s)
}

fn gen_hex_string(max: u32) -> String {
    let s = rnd_str("01234456789ABCDEF", max);
    format!("<{}>", s)
}

fn gen_name(max: u32) -> String {
    let s = rnd_str("0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ", max);
    format!("/{}", s)
}

/*
TODO: gen_array
this requires all data types
*/

fn gen_array(max: u32) -> String {
    let len = rand_range(0, max);
    let mut s = String::from("[ ");
    for _ in 0..len {
        let c = rand::thread_rng().gen_range(0..5);
        if c == 0 {
            s.push_str(&gen_bool());
        } else if c == 1 {
            s.push_str(&gen_num());
        } else if c == 2 {
            s.push_str(&gen_string(100));
        } else if c == 3{
            s.push_str(&gen_name(30));
        } else {
            s.push_str(&gen_dict(3));
        }
        s.push(' ');
    }
    s.push_str(" ]"); s

}


fn gen_dict(max: u32) -> String {
    let len = rand_range(0, max);
    let mut s = String::from("<< ");
    for _ in 0..len {
        s.push_str(&gen_name(30));
        s.push(' ');
        let c = rand::thread_rng().gen_range(0..5);
        if c == 0 {
            s.push_str(&gen_bool());
        } else if c == 1 {
            s.push_str(&gen_num());
        } else if c == 2 {
            s.push_str(&gen_string(100));
        } else if c == 3{
            s.push_str(&gen_name(30));
        } else {
            s.push_str(&gen_dict(3));
        }
        s.push('\n');
    }
    s.push_str(">>"); s
}

fn gen_bytes(max: u32) -> String {
    let len = rand_range(0, max);
    let mut buf = String::new();
    for _ in 0..len {
        buf.push(char::from_u32(rand_range(0, 257)).unwrap())
    }
    buf
}

