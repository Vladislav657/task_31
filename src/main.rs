use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn find_max_digit_sum(file: File) -> i32{
    let reader = BufReader::new(file);

    let mut max = 0;
    let mut max_sum = 0;

    let mut current;
    let mut current_sum;

    for line in reader.lines() {
        let line = line.unwrap();
        current = line.parse::<i32>().unwrap();
        current_sum = digit_sum(current);

        if current_sum > max_sum || current_sum == max_sum && current > max{
            max_sum = current_sum;
            max = current;
        }
    }
    max
}


fn digit_sum(mut n: i32) -> i32{
    if n < 0 {
        n *= -1;
    }
    let mut sum = 0;
    while n > 0 {
        sum += n % 10;
        n /= 10;
    }
    sum
}


fn main() -> std::io::Result<()> {
    let file = File::open("SUMDIGIT.IN")?;
    let max_digit_sum = find_max_digit_sum(file);

    let mut res = File::create("SUMDIGIT.OUT")?;
    res.write_all(max_digit_sum.to_string().as_bytes())?;

    println!("SUMDIGIT.OUT: {}", max_digit_sum);
    Ok(())
}
