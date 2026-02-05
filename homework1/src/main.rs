fn is_even(n: i32) -> bool {
    true
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    let newTemp = (f - 32.0) / 1.8;
    newTemp
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    let newTemp = (c * 1.8) + 30.0;
    newTemp
}

fn main() {
    // Part 1
    //
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

    // Part 2
    let nums = [5, 10, 15, 20, 25, 30, 35, 40, 45, 50];
}
