fn collatz(curr: i32, cnt: i32) {
    if cnt > 0 {
        println!("{} -> {}", cnt, curr);
    }
    let next = if (curr % 2) == 0 {
        curr / 2
    } else {
        curr * 3 + 1
    };
    if curr == 1 {
        return;
    } else {
        collatz(next, cnt + 1);
    };
}

fn main() {
    collatz(5, 0);
}
