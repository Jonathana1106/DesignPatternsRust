
trait Car {
    fn check_engine(&self);
}

trait Motorcycle {
    fn check_brakes(&self);
}

trait Factory<C: Car, M: Motorcycle> {
    fn new_car(&self) -> C;
    fn new_motorcycle(&self) -> M;
}


struct Civic;

impl Car for Civic {
    fn check_engine(&self) {
        println!("The Honda Civic engine is ready!");
    }
}

struct Tornado;

impl Motorcycle for Tornado {
    fn check_brakes(&self) {
        println!("The Honda Tornado brakes is ready!");
    }
}


struct HondaFactory;

impl Factory<Civic, Tornado> for HondaFactory {
    fn new_car(&self) -> Civic {
        return Civic;
    }

    fn new_motorcycle(&self) -> Tornado {
        return Tornado;
    }
}


struct Tourer;

impl Car for Tourer {
    fn check_engine(&self) {
        println!("The BMW Tourer engine is ready!");
    }
}

struct F800GT;

impl Motorcycle for F800GT {
    fn check_brakes(&self) {
        println!("The BMW F800GT brakes is ready!");
    }
}


struct BMWFactory;

impl Factory<Tourer, F800GT> for BMWFactory {
    fn new_car(&self) -> Tourer{
        return Tourer;
    }

    fn new_motorcycle(&self) -> F800GT {
        return F800GT;
    }
}


fn main() {

    let honda = HondaFactory;
    let bmw = BMWFactory;

    let car = honda.new_car();
    car.check_engine();

    let car = bmw.new_car();
    car.check_engine();

    let motorcycle = honda.new_motorcycle();
    motorcycle.check_brakes();

    let motorcycle = bmw.new_motorcycle();
    motorcycle.check_brakes();
}
