use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let List(ref vec) = *self;
        let len = vec.len();

        for (count, v) in vec.iter().enumerate() {
            if count < len - 1 { try!(write!(f, "{}, ", v)) }
        }

        write!(f, "{}", vec[len-1])
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
