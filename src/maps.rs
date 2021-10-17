fn main() {
    let items = (1..10).into_iter();
    let other_items: Vec<i32> = iter.map(|x| {
        x + 1
    }).collect();

    let items2 = (1..10).into_iter();
    let mut other_items2 = Vec::new();
    for item in items2 {
        other_items2.push(x + 1);
    }
}

fn lazy_map() {
    let items = (1..10).into_iter();
    let other_items: Vec<i32> = items
        .map(|x| {foor(x, 12)})
        .map(|x| {bar(x, "Hello")})
        .map(|x| {baz(x)})
        .map(|x| {fuz(x, bar(x))})
        .collect();
}

fn map_filter() {
    let items = (1..10).into_iter();
    let even: Vec<_> = items
        .filter(|x| {x %2 == 0})
        .collect();

}

fn map_fold() {
    let items = (1..10).into_iter();
    item.fold(0, |sum, item| {
        sum + item
    });
    //45

}

fn map_combin() {
    let items = (1..10).into_iter();
    items.filter(|x| {x % 2 == 1})
    .map(|x| { x * x})
    .filter(|x | {x % 5 1= 0})
    .fold(0, |sum, x| {
        sum + x
    });

}