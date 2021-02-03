fn main() {
    give();
    lender();
    use_push_all();
}

fn give() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    take(vec);
    //vec.push(3);
}

fn take(vec: Vec<i32>) {
    println!("{:?}", vec);
}

fn lender() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    use_vector(&vec);
}

fn use_vector(vec: &Vec<i32>) {
    //vec.push(3);
    //vec[1] += 1;
    println!("{:?}", vec);
}

fn use_push_all() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2); 
    //push_all(&vec, &mut vec);
    let mut vec2 = Vec::new();
    push_all(&vec, &mut vec2);
    println!("{:?} {:?}", vec, vec2);
}

fn push_all(from: &Vec<i32>, to: &mut Vec<i32>) {
    for elem in from.iter() {
        to.push(*elem);
    }
}

fn push_all2(from: &Vec<i32>, to: &mut Vec<i32>) {
    for i in 0.. from.len() {
        let elem = &from[i];
        to.push(*elem);
    }
}