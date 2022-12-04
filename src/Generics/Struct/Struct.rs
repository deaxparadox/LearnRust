pub struct S<T1, T2> {
    c: char,
    n1: T1, 
    n2: T1,
    n3: T2,
}

struct SE<T1, T2> (char, T1, T1, T2);

pub fn GenericStruct() {
    // struct
    let _s = S::<u16, f32> { c: 'a', n1: 34, n2: 782, n3: 0.02};

    // tuple struct
    let _se = SE::<u16, f32> ('a', 34, 782, 0.02);
}

pub fn f<T>(ch: char, num1: T, num2: T) -> T {
    if ch == 'a' {
        num1
    } else {
        num2
    }
    
}
