pub fn parse_calories(input: &str) -> impl Iterator<Item = u32> + '_ {
    input.split("\n\n").map(|x| {
        let total_calories: u32 = x.lines().map(|cal| cal.parse().unwrap_or(0)).sum();
        total_calories
    })
}
