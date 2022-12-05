trait HasSquareRoot {
    fn sq_root(self) -> Self;
}

impl HasSquareRoot for f32 {
    fn sq_root(self) -> Self {
        f32::sqrt(self)
    }
}

impl HasSquareRoot for f64 {
    fn sq_root(self) -> Self {
        f64::sqrt(self)
    }
}

fn quartic_root<Number>(x: Number) -> Number
where Number: HasSquareRoot {
    x.sq_root().sq_root()
}

pub fn quartic()  {
    println!("{} {}", quartic_root(100f64), quartic_root(100f32));
}


fn _f1<T>(a: T) -> T { a } 
fn _f2<T>(a: T) -> T { 
    let b: T = a;
    let mut c = b;
    c = _f1(c);
    c
}
fn _f3<T>(a: &T) -> &T { a }

pub fn Swap() {
    let mut a = 'A';
    let mut b = 'B';
    println!("{}, {}", a, b);
    std::mem::swap(&mut a, &mut b);
    println!("{}, {}", a, b);
}