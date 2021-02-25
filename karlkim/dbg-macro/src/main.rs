// https://doc.rust-lang.org/edition-guide/rust-next/dbg-macro.html
// https://doc.rust-lang.org/std/macro.dbg.html
fn main() {
    let mut numbers = [3, 1, 2];
    dbg!(numbers);
    numbers.sort();
    dbg!(numbers);
    dbg!(numbers[0] + 1);
    dbg!(1usize, 2u32);
    let i = 10;
    let n = 2;
    let j = 11;
    let (x, y) = dbg!(i*n, j+1);
    dbg!(x, y);
    let (i, j) = dbg!((3, 5));
    let (k, l) = dbg!("{:?}", (3, 5));
    dbg!(i, j, k, l);

    factorial_dbg(5);
}


fn factorial(n: u32) -> u32 {
    if n <= 1 {
        n
    } else {
        n * factorial(n - 1)
    }
}
// Because the dbg! macro returns the value of what it's debugging, instead of eprintln! which returns ().
fn factorial_dbg(n: u32) -> u32 {
    if dbg!(n <= 1) {
        dbg!(1)
    } else {
        dbg!(n * factorial(n - 1))
    }
}