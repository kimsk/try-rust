// https://github.com/rustomax/rust-iterators#creating-your-own-iterators
// FahrToCelc iterator
// a very simple iterator that produces a series of pairs of temperatures (Fahrenheit, Celsius)
// add count to avoid infinite
#[derive(Debug)]
struct FahrToCelc {
    fahr: f32,
    step: f32,
    count: i32,
}

impl FahrToCelc {
    fn new(fahr: f32, step: f32) -> FahrToCelc {
        FahrToCelc {
            fahr: fahr,
            step: step,
            count: 0,
        }
    }
}

// impl Iterator for T
impl Iterator for FahrToCelc {
    type Item = (f32, f32);
    fn next(&mut self) -> Option<Self::Item> {
        if self.count >= 1000 {
            None
        } else {
            self.count += 1;
            let curr_fahr = self.fahr;
            let curr_celc = (self.fahr - 32.0) / 1.8;
            self.fahr = self.fahr + self.step;
            Some((curr_fahr, curr_celc))
        }
    }
}

pub fn get_max_temperatures() {
    let mut ftc = FahrToCelc::new(0.0, 5.0);
    for (f, c) in &mut ftc {
        println!("{:7.2} °F = {:7.2} °C", f, c);
    }
    dbg!(ftc);

    // `&FahrToCelc` is not an iterator
    // the trait `Iterator` is not implemented for `&FahrToCelc`

    // `Iterator` is implemented for `&mut create_iter::FahrToCelc`,
    // but not for `&create_iter::FahrToCelc`

    // required because of the requirements on the impl of `IntoIterator` for `&FahrToCelc`
    // required by `into_iter`
    // let ftc = FahrToCelc::new(0.0, 5.0);
    // for (f, c) in &ftc {
    //     println!("{:7.2} °F = {:7.2} °C", f, c);
    // }
}

pub fn get_5_temperatures() {
    // pass the starting temperature and step to the initializer function
    let ftc = FahrToCelc::new(0.0, 5.0);

    // produce the iterator table of first 5 values
    let temp_table = ftc.take(5);

    // print out the temperature table nicely
    for (f, c) in temp_table {
        println!("{:7.2} °F = {:7.2} °C", f, c);
    }
    //dbg!(temp_table);
}

// https://stackoverflow.com/questions/30218886/how-to-implement-iterator-and-intoiterator-for-a-simple-struct
