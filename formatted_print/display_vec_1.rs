use std::fmt;

struct List(Vec<i32>);


impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self.0.iter().map(i32::to_string).collect::<Vec<_>>().join(", ");
        write!(f, "[{s}]")
    }
}

fn main() {
    let v = List(vec![1,2,3]);
    println!("{v}");
}
