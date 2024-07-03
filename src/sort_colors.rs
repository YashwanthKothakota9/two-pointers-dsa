fn vector_to_string(values: &Vec<i32>) -> String {
    let mut res = String::from("[");
    for i in values {
        res.push_str(&format!("{} ", i));
    }

    res.pop();

    res.push(']');

    res
}

fn sort_colors(colors: &mut [i32]) -> Vec<i32> {
    let mut start = 0;
    let mut current = 0;
    let mut end = colors.len() - 1;

    while current <= end {
        if colors[current] == 0 {
            if colors[start] != 0 {
                colors.swap(start, current);
            }
            current += 1;
            start += 1;
        } else if colors[current] == 1 {
            current += 1
        } else {
            if colors[end] != 2 {
                colors.swap(current, end);
            }
            if end > 0 {
                end -= 1;
            } else {
                break;
            }
        }
    }

    colors.to_vec()
}

pub(crate) fn main() {
    let inputs = [
        vec![0, 1, 0],
        vec![1, 1, 0, 2],
        vec![2, 1, 1, 0, 0],
        vec![2, 2, 2, 0, 1, 0],
        vec![2, 1, 1, 0, 1, 0, 2],
    ];

    for (i, input) in inputs.iter().enumerate() {
        println!("{}.\tcolors: {}", i + 1, vector_to_string(input));
        println!(
            "\n\tThe sorted array is: {}",
            vector_to_string(&sort_colors(&mut input.to_vec()))
        );
        println!("\n{}", ("-").repeat(100));
    }
}
