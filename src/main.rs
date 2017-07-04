use std::io;

fn f_to_c(temp: f64) -> f64 {
    (temp - 32.0) * (5.0/9.0)
}

fn main() {
    println!("Enter the temp in F you would like to convert: ");

    let mut f_temp = String::new();

    io::stdin().read_line(&mut f_temp)
        .expect("Failed to read line");

    let f_temp: f64 = f_temp.trim().parse().unwrap();

    let c_temp = f_to_c(f_temp);

    println!("The temp in Celcius is: {}", c_temp);
}
