use std::iter;
use std::vec::IntoIter;

fn combine_vecs_exclicit_return_type(
    v: Vec<i32>,
    u: Vec<i32>
    )  -> iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>> {
    v.into_iter().chain(u.into_iter()).cycle()
}

fn combine_vecs(
    v: Vec<i32>,
    u: Vec<i32>,
    ) -> impl Iterator<Item=i32> {
    v.into_iter().chain(u.into_iter()).cycle()
}

fn main() {
    let v1 = vec![1,2,3];
    let v2 = vec![4,5];
    let mut v3 = combine_vecs(v1, v2);
    assert_eq!(Some(1), v3.next());
    assert_eq!(Some(2), v3.next());
    assert_eq!(Some(3), v3.next());
    assert_eq!(Some(5), v3.next());
    assert_eq!(Some(5), v3.next());
    println!("all done");
}
