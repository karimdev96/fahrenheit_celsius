
use crate::utils::conversor::*;
use std::process::exit;
pub fn _tabla(start:f64,end:f64) -> String {
    let mut n = start as f64;
    let mut ne_f:f64;
    println!("Centígrados (°C)\t│Fahrenheit (°F) ");
    loop { 
        println!("{:.1}\t\t\t│{:.1}", fahrenheit_celsius(n),celsius_fahrenheit(fahrenheit_celsius(n)));
        n = n+1.0-0.1;
        ne_f = format!("{n:.1}")
            .parse::<f64>()
            .unwrap();
        let ysum = ne_f
            .to_string()
            .chars()
            .last()
            .unwrap()
            .to_digit(10)
            .unwrap() as f64;
        let result:f64 = format!("0.{}", ysum)
            .parse()
            .expect("Error f64"); 
        if format!("{:.1}",ne_f).parse::<f64>().unwrap()  == end+result {exit(0);}
    }
}
