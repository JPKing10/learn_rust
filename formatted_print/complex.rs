use std::fmt;

#[derive(Debug)]
struct ComplexNum {
    real: f64,
    imag: f64,
}

impl fmt::Display for ComplexNum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

fn main() {
    let a = ComplexNum { real: 56.5, imag: 83.5 };
    println!("a: {a}");
    println!("debug a: {a:?}");
}


