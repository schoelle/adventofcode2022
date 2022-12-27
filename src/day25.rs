use radix_fmt::radix_5;

fn decode(input: &str) -> i64 {
    let base5: String = input.chars().map(|x| match x {
        '=' => '0', '-' => '1', '0' => '2', '1' => '3', '2' => '4', _ => panic!("digit?")
    }).collect();
    let zero: String = (0..input.len()).map(|_| '2').collect();
    return i64::from_str_radix(&base5, 5).unwrap() - i64::from_str_radix(&zero, 5).unwrap();
}

fn encode(input: i64) -> String {
    let value = input + i64::from_str_radix("222222222222222222222222", 5).unwrap();
    let base5 = radix_5(value).to_string();
    let snafu: String = base5.chars().map(|x| match x {
        '0' => '=', '1' => '-', '2' => '0', '3' => '1', '4' => '2', _ => panic!("digit?")
    }).collect();
    return snafu.trim_start_matches('0').to_string();
}

pub fn star1(filename: &str) {
    let mut total = 0;
    for line in super::utils::read_lines(filename) {
        let decoded = decode(&line);
        total += decoded;
    }
    println!("Star 1: {}", encode(total));
}

pub fn star2(_filename: &str) {
    println!("Star 2: {}", "Merry Christmas");
}
