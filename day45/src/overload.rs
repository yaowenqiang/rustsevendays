usestd::ops;
struct Foo;
struct Bar;

#[derive(Debug)]
struct FooBar;


#[derive(Debug)]
struct BarFoo;

impl ops::Add<Bar> for Foo {
    type Output = FooBar;

    fn add(self, _rhs: Bar) -> FooBar {
    }
}



