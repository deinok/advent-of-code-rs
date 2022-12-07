fn main() {
    let mut vec = include_str!("./input.txt")
        .split("\n\n")
        .map(|x| {
            x.split("\n")
                .filter_map(|y| y.parse::<u32>().ok())
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();
    vec.sort();
    vec.reverse();
    let result = vec.iter().take(3).sum::<u32>();
    println!("{}", result)
}
