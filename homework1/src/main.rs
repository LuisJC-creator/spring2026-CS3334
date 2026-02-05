fn fahrenheit_to_celsius(f: f64) -> f64 {
    let newTemp = (f - 32.0) / 1.8;
    newTemp
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    let newTemp = (c * 1.8) + 30.0;
    newTemp
}

fn main() {
    // vars we need
    const frzTemp:f64  = 32.00;
    let mut usrTemp = 32.00;
    
    // looping to convert the next 5 int temps
    let mut counter = 0;
    loop {
        let mut intTemp = usrTemp as i64;
        println!("{} in celsius is: {}", intTemp, fahrenheit_to_celsius(intTemp as f64));

        if counter == 5 {
            break;
        }
        counter += 1;
        usrTemp += 1.0;
        
    }

}
