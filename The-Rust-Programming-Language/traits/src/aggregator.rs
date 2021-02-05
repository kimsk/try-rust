use std::fmt::Display;

pub trait Summary1 {
    fn summarize(&self) -> String;
}
pub trait Summary2 {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
pub trait Summary3 {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub fn notify0<T: Summary2>(item: &T) {
    println!("(notify0) Breaking news! {}", item.summarize());
}
pub fn notify1(item: &impl Summary1) {
    println!("(notify1) Breaking news! {}", item.summarize());
}
pub fn notify_all(item1: &impl Summary1, item2: &impl Summary2, item3: &impl Summary3){
    println!("notify_all");
    println!("{}", item1.summarize());
    println!("{}", item2.summarize());
    println!("{}", item3.summarize());
}

pub fn notify_display(item: &(impl Summary1 + Display)) {
    println!("{} {}", item, item.summarize());
}

pub fn notify_display0<T: Summary1 + Display>(item: &T) {
    println!("{} {}", item, item.summarize());
}

pub fn notify_display_where<T>(item: &T)
    where T: Summary3 + Display {
    println!("{} {}", item, item.summarize());
}

pub fn notify_all_display<T, U, V>(item1: &T, item2: &U, item3: &V)
    where T: Summary1 + Display,
          U: Summary2 + Display,
          V: Summary3 {
    println!("notify_all");
    println!("{}", item1.summarize());
    println!("{}", item2.summarize());
    println!("{}", item3.summarize());
}