const ROW: u64 = 2978;
const COL: u64 = 3083;

fn main() {
    let num = num_at_coord(COL, ROW);

    println!("Code is {}", num);
}

fn coord_to_seq(x: u64, y: u64) -> u64 {
    // n(n+1)/2
    let xy_sum = x + y;
    let triangle_num = (xy_sum * (xy_sum - 1)) / 2;

    triangle_num - (y - 1)
}

fn num_at_coord(x: u64, y: u64) -> u64{
    let num_seq = coord_to_seq(x, y);

    let mut num: u64 = 20151125;
    for _ in 1..num_seq {
        num = (num * 252533) % 33554393;
    }

    num
}

#[test]
fn test_coord_to_seq() {
    assert!(coord_to_seq(1, 1) == 1);
    assert!(coord_to_seq(2, 2) == 5);
    assert!(coord_to_seq(4, 3) == 19);
    assert!(coord_to_seq(3, 4) == 18);
    assert!(coord_to_seq(2, 4) == 12);
}

#[test]
fn test_num_at_coord() {
    assert!(num_at_coord(1, 1) == 20151125);
    assert!(num_at_coord(2, 2) == 21629792);
    assert!(num_at_coord(4, 3) == 7981243);
    assert!(num_at_coord(3, 4) == 21345942);
    assert!(num_at_coord(2, 4) == 32451966);
}
