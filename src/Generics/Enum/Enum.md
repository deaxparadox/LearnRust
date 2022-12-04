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
        _res = Result1::Failure(0u16, 'd');   
    }
```

Generic struct [>>>](https://github.com/deaxparadox/LearnRust/blob/main/src/Generics/Generics.md)
Generic struct [>>>](https://github.com/deaxparadox/LearnRust/blob/main/src/Generics/Struct/Struct.md)