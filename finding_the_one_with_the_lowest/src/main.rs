use core::num;

fn main() {
    let text = include_str!("../log.txt");
    let number = text
        .split("\n")
        .enumerate()
        .fold((0, 0), |(j, prev), (i, str)| {
            let iter = str.split_whitespace();
            let num = iter.last().unwrap().parse::<usize>().unwrap();
            if prev < num {
                return (i + 1, num);
            };
            (j, prev)
        });
    println!("least is at line {}, with {} solutions", number.0, number.1)
}
