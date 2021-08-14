use std::env;
use chrono::prelude::*;

struct Kilowatts {
    kwh: f32,
    rate: f32,
    base_charge: f32
}

impl Kilowatts {
    pub fn new(kwh: f32, rate: f32) -> Self {
        const BASE: f32 = 14.00;
        Kilowatts { kwh: kwh, rate: rate, base_charge: BASE }
    }
    pub fn amount(&self) -> f32 {
        self.kwh * self.rate + self.base_charge
    }
}

fn parse() -> f32 {

    // Only one argument needed - Kilowatts used
    let args = env::args().nth(1);
    let kilowatts = args.expect("No arguments").parse::<f32>()
                    .ok().expect("Not a valid number");
    kilowatts
}

fn main() {
    let kwh = parse();
    let local: DateTime<Local> = Local::now();
    let current_month = local.month();

    let rate: f32;
    if current_month > 6 && current_month < 11 {
        rate = 0.10772;
    } else {
        rate = 0.10271;
    }

    let usage = Kilowatts::new(kwh, rate);

    println!("Current Duke Energy bill");
    println!("------------------------");
    println!("Base service: \t${}", usage.base_charge);
    println!("Months rate: \t${}", usage.rate);
    println!("Kilowatts\\hr: \t{}", usage.kwh);
    println!("Total amount: \t${}", usage.amount());


}


