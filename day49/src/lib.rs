pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

pub trait Iterator2<T> {
    fn next(&mut self) -> Option<Self::T>;
}

struct Counter {
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        //
    }
}


