// represents 2D array in 1D
// [
//     (0,0), (0,1), (0,2)
//     (1,0), (1,1), (1,2)
//     (2,0), (2,1), (2,2)
// ]

// [0, 1, 2, 3, 4, 5, 6, 7, 8]

fn main() {
    let mut one_d: [(usize,usize);9] = Default::default();
    let len = 3;
    for idx in 0..one_d.len() {
        let (i, j) = one_to_two(idx, len);
        println!("{}, {}", i, j);
        one_d[idx] = (i, j);
    }
    dbg!(one_d);

    for i in 0..3 {
        for j in 0..3 {
            println!("{}, {} -> {}", i, j, two_to_one(i, j, len));
        }
    }

    for idx in 0..one_d.len() {
        let (i, j) = one_to_two(idx, len);
        println!("{} -> {}, {}", idx, i, j);
    }
}

fn two_to_one(i:usize, j:usize, len:usize) -> usize {
    (i*len) + j

}

fn one_to_two(idx:usize, len:usize) -> (usize, usize) {
    (idx/len, idx%len)
}
