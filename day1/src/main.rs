use std::fs;

fn process1(
    _string: &str

) -> String { 
    let lines = _string.lines();
    let mut sum = 0;

    for line in lines {
        let digits: Vec<char> = line.chars().filter(|c| c.is_digit(10)).collect();
        let first = digits.first().unwrap().to_digit(10).unwrap();
        let last = digits.last().unwrap().to_digit(10).unwrap();
        sum += first * 10 + last;
    }
    print!("{}",sum.to_string());
    return sum.to_string()
}
fn process2(
    _string: &str

) -> String { 

    let mapping = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    
    let lines = _string.lines();
    let mut sum = 0;

    for line in lines {
        let digits: Vec<char> = line.chars().filter(|c| c.is_digit(10)).collect();
        let first = digits.first().unwrap().to_digit(10).unwrap();
        let last = digits.last().unwrap().to_digit(10).unwrap();
        sum += first * 10 + last;
    }
    print!("{}",sum.to_string());
    return sum.to_string()
}
fn main (){
    let input = "1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet";
    let file = fs::read_to_string("input.txt").expect("Should have been able to read this file");
    assert_eq!("142", process1(input));
    assert_eq!("56506", process1(&file));

    let input2 = "two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen"

    assert_eq!("281", process2(input));
}

// #[cfg(test)]
// mod test {
//     fn part1(){

//         // assert_eq!(142, process1(input));
//     }


// }