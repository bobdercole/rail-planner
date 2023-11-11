use std::{env, ops::RangeInclusive};

const SEGMENT_GAP_LENGTH: i64 = 17;
const BEAM_LENGTH: i64 = 3;

fn main() {
    let args: Vec<String> = env::args().collect();

    let start_center_beam: i64 = args[1].parse().expect("should be i64");
    let segments: i64 = args[2].parse().expect("should be i64");
    let positive: bool = args[3].parse().expect("should be bool");

    let center_beam_padding = BEAM_LENGTH / 2;

    for segment in 0..segments {
        let offset = segment * (SEGMENT_GAP_LENGTH + BEAM_LENGTH);
        let coordinate = calculate(start_center_beam, offset, positive);

        let start_beam = calculate(coordinate, center_beam_padding, !positive);
        let end_beam = calculate(coordinate, center_beam_padding, positive);
        let beam = create_range(start_beam, end_beam, positive);
        println!("Beam: {}", format_range(beam, positive));

        let start_segment = calculate(end_beam, 1, positive);
        let end_segment = calculate(end_beam, SEGMENT_GAP_LENGTH, positive);
        let segment = create_range(start_segment, end_segment, positive);
        println!("\t Segment gap: {}", format_range(segment, positive));
    }
}

fn calculate(a: i64, b: i64, add: bool) -> i64 {
    if add {
        return a + b;
    }

    a - b
}

fn create_range(start: i64, end: i64, positive: bool) -> RangeInclusive<i64> {
    if positive {
        return start..=end;
    }

    end..=start
}

fn format_range(range: RangeInclusive<i64>, positive: bool) -> String {
    let strings = range.map(|item| item.to_string());
    let mut vector: Vec<String> = strings.collect();

    if !positive {
        vector.reverse();
    }

    vector.join(" ")
}
