type Identity = u32;

trait Table {
    fn identity(&self) -> Identity;
}

trait ForeignKey<T: Table> {
    fn fk(&self) -> Identity;
}

struct Car {
    id: Identity,
    model: String,
}

impl Table for Car {
    fn identity(&self) -> Identity {
        self.id
    }
}

struct Person {
    id: Identity,
    car: Identity,
}

impl Table for Person {
    fn identity(&self) -> Identity {
        self.id
    }
}

impl ForeignKey<Car> for Person {
    fn fk(&self) -> Identity {
        self.car
    }
}

struct DataBase {
    
}

fn main() {
    println!("Hello, world!");
}
