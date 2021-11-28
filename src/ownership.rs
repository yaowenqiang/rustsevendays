pub fn take_ownership_sum(v: Vec<i32>) -> i32 {
    let mut sum = 0;
    for val in v {
        sum += val;
    }
    sum
}

pub fn borrow_sum(v: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for val in v {
        sum += *val;
    }
    sum
}

pub fn cap_values_owned(max: i32, mut v: Vec<i32>) -> Vec<i32> {
    for index in 0..v.len() {
        if v[index] > max {
            v[index] = max;
        }
    }
    v
}

pub fn cap_values_borrow(max: i32, v: &mut Vec<i32>) {
    for index in 0..v.len() {
        if v[index] > max {
            v[index] = max;
        }
    }
}
