struct Sleep {
    naked: bool, 
    name: &'static str 
}
trait Animal {
    fn new(name: &'static str) -> Self;
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    fn take(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
    
}

impl Sheep {
    fn is_named(&self) -> bool {
        self.named
    }

    fn shear(&mut self) {
        if self.is_naked() {
            println!("{} is already naked ...", self.name());
        } else {
            println!("{} gets a haircut!", self.name);
            self.naked = true;
        }
    }
}

impl Animal for Sheep {
    fn new(name: &'static str) -> Sheep {
        Sheep {name: name, naked: false }
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaaaaah?"
        } else {
            "baaaaaah!"
        }
    }

    fn talk(&self) {
        println!("{} pauses briefly...{}", self.name, self.noise());
    }
}

fn main() {
    let mut dolly: Sheep = Animal::new("Dolly");
    dolly.talk();
    dolly.sheer();
    dolly.talk();
    
}
