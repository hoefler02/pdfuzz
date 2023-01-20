use std::collections::HashMap;
use clap::Parser;
use rand::Rng;
use std::fmt;
use hex;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Output PDF File
    #[arg(short, long)]
    output: String
}


#[derive(Debug)]
enum PDFObject {
    Boolean(bool),
    Number(Numeric),
    Str(String),
    Name(String),
    Arr(Vec<PDFObject>),
    Dict(HashMap<String, PDFObject>),
    //Stream(PDFStream),
    //Null(PDFNull)
}

#[derive(Debug)]
enum Numeric {
    UnsignedInteger(u32),
    SignedInteger(i32),
    Floating(f32)
}

impl Into<Numeric> for f32 {
    fn into(self) -> Numeric {
        Numeric::Floating(self)
    }
}

impl Into<Numeric> for u32 {
    fn into(self) -> Numeric {
        Numeric::UnsignedInteger(self)
    }
}

impl Into<Numeric> for i32 {
    fn into(self) -> Numeric {
        Numeric::SignedInteger(self)
    }
}

impl fmt::Display for Numeric {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Numeric::UnsignedInteger(i) => write!(f, "{}", i),
            Numeric::SignedInteger(i) => write!(f, "{}", i),
            Numeric::Floating(i) => write!(f, "{}", i)
        }
    }
}

impl PDFObject {
    fn boolean(b: bool) -> Self {
        PDFObject::Boolean(b)
    }
    fn number<T: Into<Numeric>>(n: T) -> Self {
        PDFObject::Number(n.into())
    }
    fn plain_string(s: &str) -> Self {
        PDFObject::Str(format!("({})", s))
    }
    fn hex_string(s: &str) -> Self {
        PDFObject::Str(format!("<{}>", hex::encode(s)))
    }
    fn name(s: &str) -> Self {
        PDFObject::Name(format!("/{}", s))
    }
    fn name_key(k: &str) -> String {
        format!("/{}", k)
    }
    fn array(a: Vec<PDFObject>) -> Self {
        PDFObject::Arr(a)
    }
    fn dict(d: HashMap<String, PDFObject>) -> Self {
        PDFObject::Dict(d)
    }
}

impl fmt::Display for PDFObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PDFObject::Boolean(b) => write!(f, "{}", b),
            PDFObject::Str(s) => write!(f, "{}", s),
            PDFObject::Number(n) => write!(f, "{}", n),
            PDFObject::Arr(a) => {
                write!(f, "[");
                for (idx, item) in a.iter().enumerate() {
                    if idx != 0 {
                        write!(f, " ");
                    }
                    write!(f, "{}", item);
                }
                write!(f, "]")
            },
            PDFObject::Name(n) => write!(f, "{}", n),
            PDFObject::Dict(d) => {
                write!(f, "<< ");
                for (key, val) in d.into_iter() {
                    write!(f, "{} {}\n", key, val);
                }
                write!(f, ">>\n")
            }
        }
    }
}


fn main() {
    let args = args::Parse();
}