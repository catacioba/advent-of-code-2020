const MOD: u64 = 20201227;

fn transform(subject: u64, loop_size: u64) -> u64 {
    let mut res = 1;
    for _ in 0..loop_size {
        res *= subject;
        res %= MOD;
    }
    res
}

fn guess_loop_size(public_key: u64) -> u64 {
    let mut loop_size = 0;
    let mut current = 1;

    while current != public_key {
        current *= 7;
        current %= MOD;
        loop_size += 1;
    }

    loop_size
}

pub fn solve_part1() {
    let door_public_key = 12092626;
    let card_public_key = 4707356;

    let door_loop_size = guess_loop_size(door_public_key);
    // let card_loop_size = guess_loop_size(card_public_key);

    let door_encryption_key = transform(card_public_key, door_loop_size);
    // let card_encryption_key = transform(door_public_key, card_loop_size);

    println!("{}", door_encryption_key);
    // println!("{}", card_encryption_key);
}
