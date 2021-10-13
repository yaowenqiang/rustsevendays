struct MasterCard {
    number: u8,
    verification: u8,
}
struct Visa {
    number: u8,
}
struct WestenUnion {
    name: String,
    verification: u8,
}
struct BitCredit {
    bitnumber: u32,
}

trait Creditcharge {
    fn charge_with_id(&self, id: u32) -> bool;
}

impl Creditcharge for BitCredit {
    fn charge_with_id(&self, id: u32) -> bool {
        id % 2 == self.bitnumber % 2
    }
}


fn main() {
    let card = BitCredit{ bitnumber: 1024 };
    let code = 4096;
    if card.charge_with_id(code) {
        println!("success.");
    } else {
        println!("Failure.");
    }
}