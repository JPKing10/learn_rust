use std::fmt;

struct List(Vec<i32>);


impl fmt::Display for List {
    // [1, 2, 3]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for (i, x) in self.0.iter().enumerate() {
            write!(f, "{i}: {x}")?;
            if i < self.0.len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1,2,3]);
    println!("{v}");
}
