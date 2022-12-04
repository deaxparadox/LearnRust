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