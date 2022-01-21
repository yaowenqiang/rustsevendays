fn used_function() {}

#[allow(dead_code)]
fn noisy_unused_function() {
    //FIXME ^ Add an attribute to suppress the warning
}
fn main() {
    used_function();
}
