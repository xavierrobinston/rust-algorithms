pub struct Sedan;
pub struct SUV;

impl LandCapable for Sedan {
    fn drive(&self) {
        println!("Sedan is driving")
    }
}

impl LandCapable for SUV {
    fn drive(&self) {
        println!("SUV is driving")
    }
}

fn road_trip_sedan(sedan: &Sedan) {
    sedan.drive();
}

fn road_trip_suv(suv: &SUV) {
    suv.drive();
}

trait LandCapable {
    fn drive(&self) {
        println!("Default drive")
    }
}

trait WaterCapable {
    fn float(&self) {
        println!("Default float")
    }
}

trait Amphibious: LandCapable + WaterCapable {}

struct Hovercraft;
impl Amphibious for Hovercraft {}
impl LandCapable for Hovercraft {}
impl WaterCapable for Hovercraft {}
/*
// dynamic dispatch
fn road_trip(vehicle: &dyn LandCapable) {
    vehicle.drive();
}
*/

// static dispatch
fn road_trip(vehicle: &impl LandCapable) {
    vehicle.drive();
}

fn traverse_frozen_lake(amp: &impl Amphibious) {
    amp.drive();
    amp.float();
}

pub fn test_polymorphism() {
    let sedan = Sedan;
    //road_trip_sedan(&sedan);
    road_trip(&sedan);

    let suv = SUV;
    //road_trip_suv(&suv);
    road_trip(&suv);

    let water = Hovercraft;
    traverse_frozen_lake(&water);
}
