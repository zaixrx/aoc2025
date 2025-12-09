fn get_input() -> String {
    let mut args = std::env::args();

    let prog_name = args.next().expect("Bro are you running on a microwave?");
    let input_path = args.next().unwrap_or_else(|| {
        eprintln!("ERROR: usage: {} <input_path>", prog_name);
        std::process::exit(69)
    });

    std::fs::read_to_string(input_path).expect("ERROR: Failed to read file.")
}

fn sum_max_joltages(input: String, joltage_size: usize) -> usize {
    if joltage_size < 1 { return 0; }

    let mut sum = 0;
    for line in input.lines() {
        if line.len() < joltage_size { continue; }

        let (mut off, mut max) = (0, 0);
        for i in 0..joltage_size {
            let (mut max_idx, mut max_val) = (0, 0);
            for (idx, chr) in line.chars().skip(off).enumerate() {
                if line.len() < (off + idx + 1) + (joltage_size - i - 1) { break; }
                let val = chr.to_digit(10).expect("ERROR: to_digit") as usize;
                if val > max_val {
                    max_idx = idx;
                    max_val = val;
                }
            }
            off += max_idx + 1;
            max = max * 10 + max_val;
        }
        sum += max;
    }

    sum
}

fn main() {
    let input = get_input();

    let sum = sum_max_joltages(input, 12);
    println!("{sum}");
}
