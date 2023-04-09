pub fn sum(a: u8, b: u8) -> Result<u8, &'static str> {
    a.checked_add(b).ok_or("ERROR: attempt to add with overflow\n")
}

pub fn diff(a: i16, b: i16) -> Result<i16, &'static str> {
    a.checked_sub(b).ok_or("ERROR: attempt to subtract with overflow\n")
}

pub fn pro(a: i8, b: i8) -> Result<i16, &'static str> {
    let res = a as i16 * b as i16;
    if res < i8::MIN as i16 || res > i8::MAX as i16 {
        Err("ERROR: attempt to multiply with overflow\n")
    } else {
        Ok(res as i8 as i16)
    }
}

pub fn quo(a: i32, b: i32) -> i32 {
    a / b
}

pub fn rem(a: f32, b: f32) -> f32 {
    a % b
}

fn main() {
    
    match sum(234, 2) {
        Ok(resultat) => println!("sum: {}", resultat),
        Err(e) => println!("{}", e),
    } 
    match sum(1, 255) {
        Ok(resultat) => println!("sum: {}", resultat),
        Err(e) => println!("{}", e),
    } 
    
    match diff(234, 2) {
        Ok(resultat) => println!("diff: {}", resultat),
        Err(e) => println!("{}", e),
    }
    match diff(-32768, 32766) {
        Ok(resultat) => println!("diff: {}", resultat),
        Err(e) => println!("{}", e),
    } 
    
    match pro(23, 2) {
        Ok(resultat) => println!("pro: {}", resultat),
        Err(e) => println!("{}", e),
    } 
    match pro(-128, 2) {
        Ok(resultat) => println!("pro: {}", resultat),
        Err(e) => println!("{}", e),
    } 
  
    println!("quo: {}", quo(22, 2)); 
    println!("quo: {}\n", quo(-128, 2)); 
    
    println!("rem: {}", rem(22.0, 2.0)); 
    println!("rem: {}", rem(-128.23, 2.0)); 
}