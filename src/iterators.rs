//x.into_iter() gives an iterator over T
//x.iter_mut() gives an iterator over &mut T
//x.iter() gives an iterator over &T
//iterator are lazy

fn main() {
    let mut iterator = (1..5).into_iter();
    iterator.skip(2);
    iterator.next();
    let taken = iterator.take(2);
    iterator.next();
    iterator.next();
    iterator.next();
    iterator.next();
    iterator.next();
    iterator.next();

    let mut iterator2 = vec![
        "A",
        "B",
        "C",
        "A",
    ].into_iter();

    // The enumerate() method adds indices to the elements iterated over
    // This is especially useful when iterating over slices, vectors and arrays

    let enumerated = iterator.enumerate();
    enumerated.next();//some("0", "A")
    enumerated.next();
    

    // the colect() Method turns an iterator into a Vec or some other collection that implements the Fromlter trait

    let mut iterator3 = (1..10).into_iter();
    iterator3.skip(2);
    let taken = iterator3.take(4);
    let v: Vec<i32> = taken.collect();
    //vec![3,44,55.6]


}
