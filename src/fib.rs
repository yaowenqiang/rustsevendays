use std::collections::HashMap;
const FIB_ONE: u64 = 1;
const FIB_TWO: u64 = 1;

fn fib(n: u64) -> u64 {
    if n == 0 {
        FIB_ONE
    } else if n == 1 {
        FIB_TWO
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

pub fn fib_dym(n: u64, map: &mut HashMap<u64, u64>) -> u64 {
    match n {
        0 | 1 => 1,
        n => {
            if map.contains_key(&n) {
                *map.get(&n).unwrap()
            } else {
                let val = fib_dym(n - 1, map) + fib_dym(n - 2, map);
                map.insert(n, val);
                val
            }
        }
    }
}
