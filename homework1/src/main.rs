fn check_guess(guess: i32, secret: i32) -> i32 {

}

fn is_even(n: i32) -> bool {
    if(n % 2 == 0){
        return true;
    }
    else {
        return false;
    }
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
    // PART 1
    //
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

    // PART 2
    //
    //


    let nums = [5, 10, 15, 20, 25, 30, 35, 40, 45, 50];
    counter = 0;
    loop {
        
        //even or odd
        if(is_even(nums[counter])){
            println!("{} is even!", nums[counter]);
        }
        else{
            println!("{} is odd! ", nums[counter]);
       }
       
       // FizzBuzz
        if nums[counter] % 5 == 0 && nums[counter] % 3 == 0 {
            println!("FizzBuzz");
        }            
        else if nums[counter] % 3 == 0 {
           println!("Fizz");
        }
        else if (nums[counter] % 5 == 0){
            println!("Buzz");
        }
        else{
          // Nothing, divisible by neither
        }
        counter += 1;
        if counter == 10 {
            break;
        }

    }

    // summation
    counter = 0;
    let mut sum = 0;
    while counter < nums.len(){
        sum += nums[counter];
        counter += 1;
    }
    println!("Sum of all numbers in the array is: {}", sum);

    // largest number
    let mut largest = 0;
    for idx in 0..nums.len(){
        if nums[idx] >= largest{
            largest = nums[idx];
        }
    }
    println!("The largest element in the array is: {}", largest);

    // PART 3
    //
    //

    let mut secret = 40;
    

}
