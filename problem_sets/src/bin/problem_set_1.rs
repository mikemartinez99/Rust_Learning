// Declare challenge module
mod challenge;

fn main() {
    // Problem 1: temperature converter
    let farenheit: f64;
    farenheit = 80.0;
    let celsius: f64;
    celsius = celsius_to_farenheit(farenheit);
    println!("{farenheit} degrees F is {celsius} degrees C");

    // Problem 2: Even or odd calculator
    let num_odd: i32 = 5;
    let num_even: i32 = 4;
    let q2_res_odd: bool = even_or_odd(num_odd);
    let q2_res_even: bool = even_or_odd(num_even);
    println!("is {num_odd} even? {}", q2_res_odd);
    println!("is {num_even} even? {}", q2_res_even);

    // Problem 3: Simple calculator
    let (sum, product): (f64, f64);
    (sum, product) = calculator(4.0, 7.0);
    println!("Sum: {}, Product: {}", sum, product);

    // Problem 4: Mutability
    let mut x: i64 = 2;
    println!("{x}");
    {
        x = x*2;
        println!("{x}");
    }
    println!("{x}");

    // Problem 5: Score calculator
    let score_vector = vec![20, 40, 50, 60, 80, 100];
    for n in &score_vector {
        let score_res: &'static str = scores(*n as f64);
        println!("{score_res}")
    }

    // Call challenge function
    challenge::run()
    
}

// Problem 1: Temperature converter
fn celsius_to_farenheit(f: f64) -> f64 {
    //C = (F - 32.0) * 5.0 / 9.0
    (f - 32.0) * 5.0 / 9.0
}

// Problm 2: Even or odd calculator
fn even_or_odd(x: i32) -> bool {
    x % 2 == 0
}

// Problem 3: Simple calculator
fn calculator(x: f64, y: f64) -> (f64, f64) {
    (x + y, x*y)
}

// Problem 5: Score classifier
fn scores(x: f64) -> &'static str {
    if x >= 90.0 {
        "Excellent"
    } else if x >= 75.0 {
        "Good"
    } else if x >= 50.0 {
        "Pass"
    } else {
        "Fail"
    }
}
