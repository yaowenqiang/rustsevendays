#[derive(Debug, PartialEq)]
enum Animal {
    Dog, 
    Cat,
}

enum Relationship {
    Father,
    Mother,
    Doughter,
    Son,
    Sibling,
    Other,
}
enum Action {
    Drive,
    Pickup,
    Turn(Direction),
    Stop,
}

enum Direction {
    Left,
    Right,
}


fn print_action(a:Action) {
    match a {
        Action::Drive => println!("drive forward!"),
        Action::Pickup => println!("Pickup object!"),
        Action::Turn(direction) => match direction {
            Direction::Left => println!("Turn left"),
            Direction::Right => println!("Turn right"),
        },
        Action::Stop => println!("Stop!"),
    }
}

fn main() {

    // let my_cat = Animal::Cat;
    // let other_pet = Animal::Dog;
    // assert_eq!(my_cat, other_pet);
    use self::Action::*;

    let program = vec![
        Drive,
        Turn(Direction::Left),
        Drive,
        Action::Pickup,
        Turn(Direction::Left),
        Turn(Direction::Left),
        Turn(Direction::Left),
        Turn(Direction::Left),
        Drive,
        Turn(Direction::Right),
        Drive,
        Stop,
    ];
    for action in program {
        print_action(action);
    }
}