# Generics

## Generics: abstract stand-ins for concrete types or other properties.

Rust performs a struct data type check, so when you define a fucntion that uses a argument of a certain type, say

```rs
    fn square_root(x: f32) -> f32 {}

    square_root(45.2f32)
```

You cannot pass a different type, like 42.2f64. As Rust has many different numeric types, when you write a function, you must scope with the problem of which type to choose.


## Defining and Using Generic Functions 

```rs 
    // Library code 
    fn f<T>(ch: char, num1: T, num2: T) -> T {
        if ch == 'a' { num 1 }
        else { num2 }
    }

    // Application code 
    let a: i16 = f::<i16>('a', 37, 41);
    let b: i16 = f::<f64>('b', 37.2, 41.1);
    println!("{} {}", a, b);
```

```sh 
    37
```
