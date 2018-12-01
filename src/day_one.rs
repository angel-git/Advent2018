use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

fn read_file() -> String {
    let filename = "./res/day_one_input.txt";
    println!("In file {}", filename);

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    return contents;
}

fn part1() -> i32 {

    let contents = read_file();

    let mut count = 0;
    for (_num, line) in contents.lines().enumerate() {
        let value = line.trim().parse::<i32>().unwrap();
        count = count + value;
    }

    return count;
}

fn part2() -> i32 {

    let contents = read_file();

    let mut numbers: HashSet<i32> = HashSet::new();
    let mut count = 0;
    let mut found = false;
    while !found {
        for (_num, line) in contents.lines().enumerate() {
            let value = line.trim().parse::<i32>().unwrap();
            count = count + value;
            found = !numbers.insert(count);
            if found {
                break;
            };
        }
    }
    return count;
}

#[cfg(test)]
mod day_one_tests {

    use::day_one::part1;
    use::day_one::part2;

    #[test]
    fn part1test() {
        assert_eq!(484, part1());
    }

    #[test]
    fn part2test() {
        assert_eq!(367, part2());
    }
}

