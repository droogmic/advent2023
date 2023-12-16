#[cfg(test)]
mod tests {
    use test_log::test;

    use crate::{get_days, Part, PrimaryExample};

    const EXAMPLE_ANSWERS: [[&'static str; 2]; 6] = [
        ["142", "281"],
        ["8", "2286"],
        ["4361", "467835"],
        ["13", "30"],
        ["35", "46"],
        ["288", "71503"],
    ];

    #[test]
    fn test_days_examples() {
        let days = get_days();
        for (day_num, day) in days {
            let (part1, part2) = match day.get_examples() {
                PrimaryExample::Same(example) => day.both(example).unwrap(),
                PrimaryExample::Different([first, second]) => (
                    day.calc(Part::First, first).unwrap(),
                    day.calc(Part::Second, second).unwrap(),
                ),
            };
            let expected = EXAMPLE_ANSWERS
                .get(day_num - 1)
                .expect(&format!("answer should be provided for day {}", day_num));
            let expected_part1 = expected[0];
            let expected_part2 = expected[1];
            assert_eq!(
                part1, expected_part1,
                "day {day_num} part 1 example mismatch"
            );
            assert_eq!(
                part2, expected_part2,
                "day {day_num} part 2 example mismatch"
            )
        }
    }
}
