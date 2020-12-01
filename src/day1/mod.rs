use itertools::Itertools;

pub fn part1() -> Option<usize> {
    let input = include_str!("input")
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    if let Some((&x, &y)) = input
        .iter()
        .cartesian_product(input.iter())
        .find(|(&x, &y)| x + y == 2020)
    {
        Some(x * y as usize)
    } else {
        None
    }
}

pub fn part2() -> Option<usize> {
    let input = include_str!("input")
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    if let Some(((&x, &y), &z)) = input
        .iter()
        .cartesian_product(input.iter())
        .cartesian_product(input.iter())
        .find(|((&x, &y), &z)| x + y + z == 2020)
    {
        Some(x * y * z as usize)
    } else {
        None
    }
}
