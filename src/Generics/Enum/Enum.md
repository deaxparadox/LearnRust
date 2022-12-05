## Generic Enums

In Rust, enums also be generic.

```rs
    pub enum Result1<SuccessCode, FailureCode> {
        Success(SuccessCode),
        Failure(FailureCode, char),
        Uncertainty,
    }

    pub fn GenericEnum() {
        let mut _res = Result1::Success::<u32,u16>(12u32);
        _res = Result1::Uncertainty;
        // _res = Result1::Failure(0u16, 'd');

        match _res {
            Result1::Success(num) => println!("{}", num),
            Result1::Uncertainty => println!("Uncertainty"),
            Result1::Failure(a, b) => println!("{}, {}", a, b)
        }
        
    }
```

## **Exception** and **null** value

Example: The function **pop** removes the last item from a vector, and returns the removed item, if that vector contains some items. 

#####  Removing an item from an empty vector!

Some languages leaves this behavior **undefined**, some languages raise an **exception**, to be handled by an enclosing block or by the callers of the current function, leading to crash, and some languages return a specific **null** value. Rust does not use the concept of **undefined**, or **exception**, or **null**.

Here is the Rust solution:

```rs
    pub fn ExceptionNullValues() {
        let mut v = vec![11, 22, 33];
        for _ in 0..5 {
            let item: Option<i32> = v.pop();
            match item {
                Some(number) => println!("{}, ", number),
                None => println!("#, "),    
            }
        }
    }
```

Such generic type is defined by the Rust standard library as this:

```rs
enum Option<T> {
    Some(T),
    None,
}
```

This enums means: "This ia an optional value of **T** type. It has the option of being a **T** and the option os being nothing. It can be something or nothing. If it is something, it is a **T**".


## Error Handling 

The Rust standard library also defines a generic enum to handle the case in which a function cannot return a value of the expected type.


The definition of this generic enum in the standard library is:

```rs
enum Result<T, E> {
    Ok(T),
    Error(E),
}
```

## Enum standard Utility Functions 

The standard library contains some utility functions to ease the decoding of an **Option** or a **Result** value.


Generic [>>>](https://github.com/deaxparadox/LearnRust/blob/main/src/Generics/Generics.md)

Generic struct [>>>](https://github.com/deaxparadox/LearnRust/blob/main/src/Generics/Struct/Struct.md)