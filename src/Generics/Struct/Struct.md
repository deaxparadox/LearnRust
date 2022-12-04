
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


Generic enum [>>>](https://github.com/deaxparadox/LearnRust/blob/main/src/Generics/Enum/Enum.md)