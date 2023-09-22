pub fn fahrenheit_celsius(x:f64) -> f64 {
    let f = (x - 32.0)*(5f64/9f64);  
    f
}
pub fn celsius_fahrenheit(x:f64) -> f64 {
    let c = (x)*(9f64/5f64)+32f64;
    c
}
