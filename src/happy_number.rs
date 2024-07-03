fn sum_of_squared_digits(number: i32) -> i32 {
    let mut total_sum = 0;
    let mut num = number;
    while num > 0 {
        let digit = num % 10;
        num = (num as f64 / 10.0).floor() as i32;
        total_sum += digit * digit;
    }
    total_sum
}

fn is_happy_number(n: i32) -> bool {
    let mut slow_pointer = n;
    let mut fast_pointer = sum_of_squared_digits(n);

    while fast_pointer != 1 && slow_pointer != fast_pointer {
        slow_pointer = sum_of_squared_digits(slow_pointer);
        fast_pointer = sum_of_squared_digits(sum_of_squared_digits(fast_pointer));
    }

    fast_pointer == 1
}

pub(crate) fn main() {
    let inputs = [1, 5, 19, 25, 7];
    for (i, &input) in inputs.iter().enumerate() {
        println!("{}.\tInput number: {}", i + 1, input);
        let result = is_happy_number(input);
        println!("\n\tIs it a happy number? {}", result);
        println!("{}", "-".repeat(100));
    }
}
