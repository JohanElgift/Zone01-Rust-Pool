pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let exponantial = (c as f64).exp();
    let ln_abs = (c.abs() as f64).ln();
    (c, exponantial, ln_abs)
}

pub fn str_function(a: String) -> (String, String) {
    let exp_str = a
        .split_whitespace()
        .map(|s| s.parse::<f64>().unwrap().exp().to_string())
        .collect::<Vec<_>>()
        .join(" ");
    (a, exp_str)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let ln_abs = b.iter().map(|&x| (x.abs() as f64).ln()).collect();
    (b, ln_abs)
}

fn main() {
    let a: i32 = 0;
    let b = String::from("1 2 4 5 6");
    let c = vec![1, 2, 4, 5];

    println!("{:?}", nbr_function(a));
    println!("{:?}", str_function(b));
    println!("{:?}", vec_function(c));
}