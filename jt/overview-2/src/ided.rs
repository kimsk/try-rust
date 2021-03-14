pub trait Ided {
    fn my_id(&self) -> i64; 
}

pub fn use_ided(x: impl Ided) {
    println!("{}", x.my_id());
}

pub fn use_ided_generic<T: Ided>(x: T) {
    println!("{}", x.my_id());
}

pub fn use_ided_box(x: Box<dyn Ided>) {
    println!("{}", x.my_id());
}

