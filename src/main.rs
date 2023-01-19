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
}

/*
    PART 1: Header
    Gens a random PDF header
    %PDF-1.1 through %PDF-1.7 (valid) or wildcard
*/
fn gen_header(wildcard: Option<f32>) -> String {
    if wildcard.is_none() {
        let ver: u8 = rand::thread_rng().gen_range(0..8);
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
    let opt: usize = rand::thread_rng().gen_range(0..3);
    if opt == 0 {
        gen_int(false)
    } else if opt == 1 {
        gen_int(true)
    } else {
        gen_float()
    }
}

fn gen_string(max: usize) -> String {
    if rand::random() {
        gen_raw_string(max)
    } else {
        gen_hex_string(max)
    }
}

fn rnd_str(alpha: &str, max: usize) -> String {
    let alpha: Vec<char> = alpha.chars().collect();
    let len: usize = rand::thread_rng().gen_range(0..max);
    let mut res = String::new();
    for i in 0..len {
        res.push(*alpha.choose(&mut rand::thread_rng()).unwrap());
    }
    res
}

fn gen_raw_string(max: usize) -> String {
    let s = rnd_str("0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ", max);
    format!("({})", s)
}

fn gen_hex_string(max: usize) -> String {
    let s = rnd_str("01234456789ABCDEF", max);
    format!("<{}>", s)
}

fn gen_name(max: usize) -> String {
    let s = rnd_str("0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ", max);
    format!("/{}", s)
}

/*
TODO: gen_array
this requires all data types
*/

//fn gen_dict(max: usize) -> String {
//    let s = String::new();
//
//}
