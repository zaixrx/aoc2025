use std::fs;

// NOTE: that took me an embarissingly long time
fn main() {
    let mut args = std::env::args();

    let prog_name = args.next().expect("Bro are you running on a microwave?");
    let input_path = args.next().unwrap_or_else(|| {
        eprintln!("ERROR: usage: {} <input_path>", prog_name);
        std::process::exit(69)
    });
    
    let input: String = fs::read_to_string(input_path).expect("Failed to read file ../input.txt");

    let mut base = 50;
    let mut pwd  = 0;
    for line in input.lines() {
        let bytes = line.as_bytes();
        let num = str::from_utf8(&bytes[1..]).expect("invalid number string").
            parse::<i32>().expect("invalid number string") * match bytes[0] as char {
            'R' => 1,
            'L' => -1,
            _ => unreachable!("Expect L/R")
        };

        let target = base + num;
        if 0 <= target && target < 100 {
            base = {
                if target == 0 {
                    pwd += 1;
                }
                target
            };
            continue;
        } 
        if base != 0 && target < 0 { pwd += 1; }
        pwd += target.abs() / 100;
        base = target.rem_euclid(100);
    }

    println!("Password is: {pwd}");
}
