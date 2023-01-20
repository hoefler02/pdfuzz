use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::fmt;
use hex;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PDFObject {
    Boolean(bool),
    Number(Numeric),
    Str(String),
    Name(String),
    Arr(Vec<PDFObject>),
    Dict(HashMap<PDFObject, PDFObject>),
    Stream(HashMap<PDFObject, PDFObject>, String),
    Null
}

#[derive(Debug, Clone)]
pub enum Numeric {
    UnsignedInteger(u32),
    SignedInteger(i32),
    Floating(f32)
}

#[derive(Debug)]
pub struct Indirect {
    obj_num: u32,
    gen_num: u32,
    obj: PDFObject
}

impl Indirect {
    pub fn new(n: u32, g: u32, o: PDFObject) -> Self {
        Indirect {
            obj_num: n,
            gen_num: g,
            obj: o
        }
    }
}

impl PartialEq for Numeric {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Numeric::UnsignedInteger(a), Numeric::UnsignedInteger(b)) => a == b,
            (Numeric::SignedInteger(a), Numeric::SignedInteger(b)) => a == b,
            (Numeric::Floating(a), Numeric::Floating(b)) => (a - b).abs() < 0.0001,
            _ => false,
        }
    }
}

impl Eq for Numeric {}


impl Hash for PDFObject {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            PDFObject::Boolean(b) => b.hash(state),
            PDFObject::Number(n) => n.hash(state),
            PDFObject::Str(s) => s.hash(state),
            PDFObject::Name(s) => s.hash(state),
            PDFObject::Arr(v) => v.hash(state),
            PDFObject::Dict(map) => {
                let vec: Vec<(&PDFObject, &PDFObject)> = map.iter().collect();
                vec.hash(state);
            },
            PDFObject::Stream(map, s) => {
                let vec: Vec<(&PDFObject, &PDFObject)> = map.iter().collect();
                let mut hasher = DefaultHasher::new();
                vec.hash(&mut hasher);
                s.hash(&mut hasher);
                hasher.finish().hash(state);
            },
            PDFObject::Null => {
                let mut hasher = DefaultHasher::new();
                let variant_discriminant = std::mem::discriminant(self);
                variant_discriminant.hash(&mut hasher);
                hasher.finish().hash(state);
            }
        }
    }
}

impl Hash for Numeric {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Numeric::UnsignedInteger(n) => n.hash(state),
            Numeric::SignedInteger(n) => n.hash(state),
            Numeric::Floating(n) => n.to_bits().hash(state),
        }
    }
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
    pub fn boolean(b: bool) -> Self {
        PDFObject::Boolean(b)
    }
    pub fn number<T: Into<Numeric>>(n: T) -> Self {
        PDFObject::Number(n.into())
    }
    pub fn plain_string(s: &str) -> Self {
        PDFObject::Str(format!("({})", s))
    }
    pub fn hex_string(s: &str) -> Self {
        PDFObject::Str(format!("<{}>", hex::encode(s)))
    }
    pub fn name(s: &str) -> Self {
        PDFObject::Name(format!("/{}", s))
    }
    pub fn array(a: Vec<PDFObject>) -> Self {
        PDFObject::Arr(a)
    }
    pub fn dict(d: HashMap<PDFObject, PDFObject>) -> Self {
        PDFObject::Dict(d)
    }
    pub fn stream(d: HashMap<PDFObject, PDFObject>, s: &str) -> Self {
        PDFObject::Stream(d, s.to_string())
    }
    pub fn null() -> Self {
        PDFObject::Null
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
            },
            PDFObject::Stream(d, s) => {
                write!(f, "{}", PDFObject::Dict((*d).clone()));
                write!(f, "stream\n");
                write!(f, "{}", s);
                write!(f, "\nendstream\n")
            },
            PDFObject::Null => {
                write!(f, "null")
            }
        }
    }
}

impl fmt::Display for Indirect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} obj\n{}\nendobj", self.obj_num, self.gen_num, self.obj)
    }
}