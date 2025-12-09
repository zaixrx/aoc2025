fn get_input() -> String {
    let mut args = std::env::args();

    let prog_name = args.next().expect("Bro are you running on a microwave?");
    let input_path = args.next().unwrap_or_else(|| {
        eprintln!("ERROR: usage: {} <input_path>", prog_name);
        std::process::exit(69)
    });

    std::fs::read_to_string(input_path).expect("ERROR: Failed to read file.")
}

fn remove_accessible_rolls(rolls: &mut Vec::<String>) -> usize {
    let mut removed_count = 0;
    for i in 0..rolls.len() {
        let roll_len = rolls[i].len();
        for j in 0..roll_len {
            let mut count = 0;
            let (i, j) = (i as isize, j as isize);
            let mut y = i-1; while y <= i+1 {
                if y < 0 || y >= rolls.len() as isize {
                    y += 1;
                    continue;
                }
                let mut x = j-1; while x <= j+1 {
                    if x < 0 || x >= roll_len as isize || (y == i && x == j) {
                        x += 1;
                        continue;
                    }
                    if rolls[y as usize].as_bytes()[x as usize] == b'@' {
                        count += 1;
                    }
                    x += 1;
                }
                y += 1;
            }
            if count < 4 && rolls[i as usize].as_bytes()[j as usize] == b'@' {
                unsafe {
                    rolls[i as usize].as_bytes_mut()[j as usize] = b'x';
                }
                removed_count += 1;
            }
        }
    }
    removed_count
}

fn main() {
    let input = get_input();
    let mut rolls = Vec::<String>::new();
    for line in input.lines() {
        rolls.push(String::from(line));
    }

    let mut sum = 0;
    loop {
        let res = remove_accessible_rolls(&mut rolls);
        if res == 0 { break; }
        sum += res;
    }

    println!("{}", rolls.join("\n"));
    println!("removed: {sum}");
}
