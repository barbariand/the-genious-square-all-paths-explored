use core::num;

fn main() {
    let text = include_str!("../log2.txt");
    let number = text
        .split("\n")
        .enumerate()
        .fold((0, 0), |(j, prev), (i, str)| {
            let iter = str.split_whitespace();
            let last = iter.last().unwrap();
            println!("{}", str);
            let num = last
                .parse::<usize>()
                .expect(&format!("found invalid {}", last));
            if prev > num {
                return (i + 1, num);
            };
            (j, prev)
        });
    println!(
        "biggest is at line {}, with {} solutions",
        number.0, number.1
    )
}
