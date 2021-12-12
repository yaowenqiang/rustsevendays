fn give_adult(drink: Option<&str>) {
    match drink {
        Some("lemonade") => println!("Yuck! Too sugary."),
        Some(inner) => println!("{}? How nice.", inner),
        None          => println!("No drink? Oh well."),
    }
}


fn drink(beverage: &str) {
    if beverage == "lemonade" {
        panic!("AAAaaaaa!!!!!");
    }
    println!("Some refreshing {} is all I need.", beverage);
}

fn drink2(drunk: Option<&str>) {
    let inside = drunk.unwrap();
    if inside == "lemonade" { panic!("AAAaaaaa");}
    println!("I love {}s!!!!!", inside);
}
fn main() {
    drink("water");

    let water = Some("water");
    let lemonade = Some("lemonade");
    let void = None;
    give_adult(water);
    give_adult(lemonade);
    give_adult(void);

    let coffee = Some("coffee");
    let nothing = None;

    drink2(coffee);
    drink2(nothing);
    drink("lemonade");

}
