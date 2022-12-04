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

The __T__ word enclosed in  angular brackets. This symbol is a type parameter of the function declaration. It means that what is being declared is not a concrete function, but a generic function, which is parameterized by the __T__ type parameter. 

While the __ch__ argument is of __char__ type, the __num1__ and __num2__ arguments, as well as the function returned value are of the __T__ generic type. When such a function will be used, it will be required to replace such __T__ parameter with a concrete type, so obtaining a concrete function.

__f::\<i16\>__ function, that is the concrete function obtained by replacing the __T__ parameter with the __i16__ type. Similarly, __f::\<f64\>__ function with __f64__ type.



## Inferring the parameteric Types

If you need to parameterize a function with several values of different types, you can do that by specifying several type parameters: 

```rs
    fn f<Param1, Param2>(_a: Param1, _b: Param2) {}
    f("a", true);
    f(12.56, "Hello");
    f((3, 'a'), [5,6,7]);
```

This program is valid, event if it does nothing.


## Defining and Using Generic Structs

Parametric types are usefull also for declaring generic structs and generic tuple-structs:

```rs
    // Generic struct
    pub struct S<T1, T2> {
        c: char,
        n1: T1, 
        n2: T1,
        n3: T2,
    }

    // Generic tuple struct
    struct SE<T1, T2> (char, T1, T1, T2);

    pub fn GenericStruct() {
        // struct
        let _s = S { c: 'a', n1: 34, n2: 782, n3: 0.02};

        // tuple struct
        let _se = SE ('a', 34, 782, 0.02);
    }
```

Also for structs the type parameter concretizations can be made explicit:

```rs
    pub struct S<T1, T2> {
        c: char,
        n1: T1, 
        n2: T1,
        n3: T2,
    }

    struct SE<T1, T2> (char, T1, T1, T2);

    pub fn GenericStruct() {
        // concrete type
        let _s = S::<u16, f32> { c: 'a', n1: 34, n2: 782, n3: 0.02};

        let _se = SE::<u16, f32> ('a', 34, 782, 0.02);
    }
```