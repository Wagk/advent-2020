pub fn part1() -> usize {
    let input = include_str!("input").lines().collect::<Vec<_>>();

    input
        .into_iter()
        .filter(|password| {
            match password
                .split_ascii_whitespace()
                .collect::<Vec<&str>>()
                .as_slice()
            {
                [range, letter, password] => {
                    let (min, max) = {
                        let range = range
                            .split("-")
                            .map(|s| s.parse::<usize>().unwrap())
                            .collect::<Vec<_>>();
                        (range[0], range[1])
                    };
                    let letter = letter.strip_suffix(":").unwrap().parse::<char>().unwrap();

                    (min..=max).contains(&password.chars().filter(|&c| c == letter).count())
                }
                _ => panic!(),
            }
        })
        .count()
}
