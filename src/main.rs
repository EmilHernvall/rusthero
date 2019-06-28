fn intersperse_counts(input: &str) -> String {
    struct Compaction {
        last_idx: i32,
        accumulator: Vec<(i32, u32)>,
    }

    input
        .chars()
        .map(|x| x.to_digit(10))
        .zip(
            // Construct a sequence which is shifted one step forward, with a
            // None added to end so that we are able to process all digits.
            input
                .chars()
                .map(|x| x.to_digit(10))
                .skip(1)
                .chain(std::iter::once(None)),
        )
        .enumerate()
        // Keep only the positions where the digit changes (types added for clarity)
        .filter(|(_, (a, b)): &(usize, (Option<u32>, Option<u32>))| a != b)
        // Drop the second value as we are only interested in the first
        .filter_map(|(i, (a, _))| a.map(|x| (i as i32, x)))
        // Process the entries one by one, and calculate the number of entries
        // in sequence for each number. Build a vector of (count, num) tuples
        // along the way.
        .fold(
            Compaction {
                last_idx: -1,
                accumulator: Vec::new(),
            },
            |mut compaction, (current_idx, num)| {
                let count = current_idx - compaction.last_idx;
                compaction.last_idx = current_idx;
                compaction.accumulator.push((count, num));
                compaction
            },
        )
        // After folding, we only care about the accumulator
        .accumulator
        // Collapse the (count, num) tuples into a string
        .iter()
        .map(|(count, num)| format!("{}{}", count, num))
        .collect()
}

fn apply_repeatedly(input: &str, iterations: usize) -> String {
    let mut result = input.to_string();
    for _ in 0..iterations {
        result = intersperse_counts(&result);
    }
    result
}

fn main() {
    println!("{}", apply_repeatedly("22164224441", 40));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provided_expansions() {
        assert_eq!("11", &intersperse_counts("1"));
        assert_eq!("12", &intersperse_counts("2"));
        assert_eq!("21", &intersperse_counts("11"));
        assert_eq!("1311", &intersperse_counts("31"));
        assert_eq!("131221", &intersperse_counts("3211"));
        assert_eq!("312213", &intersperse_counts("111223"));
    }
}
