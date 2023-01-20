mod pdfobj;

use std::collections::HashMap;
use pdfobj::PDFObject;
use pdfobj::Indirect;
use clap::Parser;
use rand::Rng;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Output PDF File
    #[arg(short, long)]
    output: String,
}

fn main() {
    let args = Args::parse();
    let arr = PDFObject::Arr(vec![PDFObject::plain_string("Hello"), PDFObject::number(1337u32)]);
    let mut d: HashMap<PDFObject, PDFObject> = HashMap::new();
    d.insert(PDFObject::name("Length"), PDFObject::number(12));
    let s = "hello world!";
    let stream = Indirect::new(10, 0, PDFObject::stream(d, s));
    println!("{}", stream);
}