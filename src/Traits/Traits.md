# Traits

A *traits* defines functinality a particular type has and can share other types. We can use traits to define shared behavior in an abstract way.

```rs
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

```

**where** clause means when this function is invoked, that type passed for the **Number** type parameter is ensured to implement the **HasSquareRoot** trait, and so you can use every function belonging to such trait, but no other function.


## Generic Functions with No Trait Bounds 

```rs
fn _f1<T>(a: T) -> T { a } 
fn _f2<T>(a: T) -> T { 
    let b: T = a;
    let mut c = b;
    c = _f1(c);
    c
}
fn _f3<T>(a: &T) -> &T { a }
```

With a value of an unbounded type parmeter **T**, you can only:

* pass it as function argument, by value or by reference;

* return it from a function, by value or by reference;

* declare, initialize, or assign a local variable;


A very rare case of an important generic standard library function that does not need trait bounds is used in this program:

```rs
let mut a = 'A';
let mut b = 'B';
println!("{}, {}", a, b);
std::mem::swap(&mut a, &mut b);
println!("{}, {}", a, b);
```

The **swap** generic function can exchange the values of any two objects having the same type. Its signature is: **fn swap\<T\>(x: \&mut T, y: &mut T)**.


## Scope of Traits

