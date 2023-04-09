pub fn km_per_hour_to_meters_per_second(km_h: f64) -> f64 {
    km_h / 3.6
}
fn main() {
    let km_h = 100.0;
	let m_s = km_per_hour_to_meters_per_second(km_h);
	println!("{} km/h is equivalent to {} m/s", km_h, m_s);
}
