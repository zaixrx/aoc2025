type Range = (usize, usize);

fn get_input() -> String {
    let mut args = std::env::args();

    let prog_name = args.next().expect("Bro are you running on a microwave?");
    let input_path = args.next().unwrap_or_else(|| {
        eprintln!("ERROR: usage: {} <input_path>", prog_name);
        std::process::exit(69)
    });

    std::fs::read_to_string(input_path).expect("ERROR: Failed to read file.")
}

fn get_usize(chars: &mut impl Iterator<Item = char>) -> Option<usize> {
    if let Some(c1) = chars.next() {
        let mut n = 0;
        let mut c = c1;
        while let Some(d) = c.to_digit(10) {
            n = n * 10 + d as usize;
            if let Some(ci) = chars.next() {
                c = ci;
            } else {
                break;
            }
        }
        Some(n)
    } else {
        None
    }
}

// TODO: work with strings not with numbers
fn get_range(chars: &mut impl Iterator<Item = char>) -> Option<Range> {
    if let Some(lhs) = get_usize(chars) {
        let rhs = get_usize(chars).expect("ERROR: expected RHS.");
        Some((lhs, rhs))
    } else {
        None
    }
}

fn first_handle_range(range: Range) -> usize {
    let mut sum = 0;
    for i in range.0..range.1+1 {
        let count = 1 + (i as f32).log10() as i32;
        if  count % 2 == 1 { continue }

        let base = 10f32.powi(count / 2) as usize;
        if base != 0 && i % base == i / base {
            sum += i;
        }
    }
    sum
}

fn second_handle_range(range: Range) -> usize {
    let mut sum = 0;
    for i in range.0..range.1+1 {
        let string = i.to_string();
        let length = string.len();

        for sup in (1..1+length/2).rev() {
            if length % sup != 0 { continue; }

            let mut offset = sup;
            let mut mismatch = false;
            while !mismatch && offset + sup <= length {
                if &string[offset..offset+sup] != &string[0..sup] {
                    mismatch = true;
                }
                offset += sup;
            }

            // if `i` is repetitive
            if !mismatch {
                sum += i;
                break;
            }
        }
    }
    sum
}

fn main() {
    #[allow(dead_code)]
    enum Exo {
        First,
        Second
    }
    // let select = Exo::First;
    let select = Exo::Second;

    let input = get_input();
    let mut chars = input.chars();

    let mut sum = 0;
    while let Some(range) = get_range(&mut chars) {
        sum += match select {
            Exo::First => first_handle_range(range),
            Exo::Second => second_handle_range(range),
        };
    }

    println!("{sum}");
}
