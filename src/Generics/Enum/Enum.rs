
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

fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0. {
        Err(format!("Divide by zero"))
    } else {
        Ok(numerator/ denominator)
    }
}

pub fn show_divide(num: f64, den: f64) {
    match divide(num, den) {
        Ok(val) => println!("{} / {} = {}", num, den, val),
        Err(msg) => println!("Cannot divided {} by {}: {}", num, den, msg),
    }
}

pub fn show_divide2() {
    let r1 = divide(8., 2.);
    let r2 = divide(8.0, 0.);

    println!("{} {}", r1.is_ok(), r2.is_ok());
    println!("{} {}", r1.is_err(), r2.is_err());
    println!("{}", r1.unwrap());
    println!("{}", r2.unwrap());
}
