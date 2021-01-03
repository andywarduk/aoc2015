use day22_lib::{play, Result, Difficulty};

fn main() {
    part1();
    part2();
}

fn part1() {
    let result = play(Difficulty::Normal);

    print_result(1, &result);
}

fn part2() {
    let result = play(Difficulty::Hard);

    print_result(2, &result);
}

fn print_result(part: u8, result: &Result) {
    println!("--- Part {} ---", part);
    println!("Minimum spend : {}", result.min_spend);
    println!("Spells cast   : {:?}", result.min_spells);
    println!("Boss wins     : {}", result.boss_wins);
    println!("Player wins   : {}", result.player_wins);
}
