//! Task 3.1: Rule 90

fn main() {
    // TODO: Task 1c)
    let mut v = read_input();
    for _ in 0..20 {
        v = next_step(&mut v);
        println!("{}", v.iter().map(|e| if *e {'█'} else {' '}).collect::<String>());
    }
}

/// Reads a valid initial configuration for our automaton from the terminal.
fn read_input() -> Vec<bool> {
// fn read_input() -> String {
    // This tries to read a string from the terminal, checks whether it's
    // valid (only contains 1's and 0's). If the user fails to input a correct
    // string, this routine will ask again until the user finally manages to
    // give us a correct string.
    //
    // You don't need to understand this routine yet; that's why I've written
    // it already ;-)
    //
    // You only need to use the `input` variable (of type `String`). You can
    // also assume that it only contains '0' and '1' chars.
    let input = {
        let mut buffer = String::new();

        loop {
            println!("Please give me the initial configuration (a string of '0' and '1'!):");
            buffer.clear();

            // `read_line` returns an error if the input isn't valid UTF8 or if
            // a strange IO error occured. We just panic in that case...
            std::io::stdin()
                .read_line(&mut buffer)
                .expect("something went seriously wrong :O");

            if buffer.trim().chars().all(|c| c == '1' || c == '0') {
                break;
            }
        }

        buffer.trim().to_string()
    };

    // TODO: Task 1a)
    input.chars().map(|c| if c == '0' { false } else { true }).collect::<Vec<bool>>()
}


// TODO: Task 1b)
fn next_step(old : &[bool]) -> Vec<bool> {
    let mut new = Vec::with_capacity(old.len());
    new.push(old[1] ^ old[old.len() - 1]);
    for i in 1..old.len()-1 {
        new.push(old[i-1] ^ old[i+1]);
    }
    new.push(old[0] ^ old[old.len() - 2]);
    new
}

#[test]
fn rule90_rules() {
    assert_eq!(next_step(&[false, false, false]), vec![false, false, false]);
    assert_eq!(next_step(&[ true, false, false]), vec![false,  true,  true]);
    assert_eq!(next_step(&[ true,  true, false]), vec![ true,  true, false]);
    assert_eq!(next_step(&[ true,  true,  true]), vec![false, false, false]);
}
