use unit_converter_lib::units::Unit;

const OPTIONS: [&str; 4] = ["length", "temperature", "mass", "volume"];
const LENGTH: usize = 0;
const TEMPERATURE: usize = 1;
const MASS: usize = 2;
const VOLUME: usize = 3;
fn fail() -> ! {
    println!("usage: <{}> <unit> <value> <unit>", OPTIONS.join("|"));
    std::process::exit(1);
}
fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    println!("{:?}", args);
    if args.len() != 4 { fail(); }
    if args[0] == OPTIONS[LENGTH] {
        use unit_converter_lib::units::length::{UNITS, UnitContainer, self};
        let unit_start = UNITS::by_name(&args[1]);
        let value = args[2].parse::<f64>();
        let value = match value { Ok(v) => v, Err(_) => panic!("value should be a valid f64") };
        let unit = UnitContainer::new(unit_start, value);
        let unit_end = UNITS::by_name(&args[3]);
        let converted_number = match unit_end {
            UNITS::MM => length::MM::from(unit).0,
            UNITS::CM => length::CM::from(unit).0,
            UNITS::M => length::M::from(unit).0,
            UNITS::KM => length::KM::from(unit).0,
            UNITS::INCH => length::INCH::from(unit).0,
            UNITS::FOOT => length::FOOT::from(unit).0,
            UNITS::YARD => length::YARD::from(unit).0,
            UNITS::MILE => length::MILE::from(unit).0,
        };
        println!("{}", converted_number);
        return;
    }
    if args[0] == OPTIONS[TEMPERATURE] {
        use unit_converter_lib::units::temperature::*;
        let unit_start = UNITS::by_name(&args[1]);
        let value = args[2].parse::<f64>();
        let value = match value { Ok(v) => v, Err(_) => panic!("value should be a valid f64") };
        let unit = UnitContainer::new(unit_start, value);
        let unit_end = UNITS::by_name(&args[3]);
        let converted_number = match unit_end {
            UNITS::KELVIN => KELVIN::from(unit).0,
            UNITS::CELSIUS => CELSIUS::from(unit).0,
            UNITS::FAHRENHEIT => FAHRENHEIT::from(unit).0,
        };
        println!("{}", converted_number);
        return;
    }
    if args[0] == OPTIONS[MASS] {
        todo!();
        /*
        use unit_converter_lib::units::mass::*;
        let unit_start = UNITS::by_name(&args[1]);
        let value = args[2].parse::<f64>();
        let value = match value { Ok(v) => v, Err(_) => panic!("value should be a valid f64") };
        let unit = UnitContainer::new(unit_start, value);
        let unit_end = UNITS::by_name(&args[3]);
        let converted_number = match unit_end {
            UNITS::KELVIN => KELVIN::from(unit).0,
            UNITS::CELSIUS => CELSIUS::from(unit).0,
            UNITS::FAHRENHEIT => FAHRENHEIT::from(unit).0,
        };
        println!("{}", converted_number);
        return;
        // */
    }
    if args[0] == OPTIONS[VOLUME] {
        todo!();
        /*
        use unit_converter_lib::units::volume::*;
        let unit_start = UNITS::by_name(&args[1]);
        let value = args[2].parse::<f64>();
        let value = match value { Ok(v) => v, Err(_) => panic!("value should be a valid f64") };
        let unit = UnitContainer::new(unit_start, value);
        let unit_end = UNITS::by_name(&args[3]);
        let converted_number = match unit_end {
            UNITS::KELVIN => KELVIN::from(unit).0,
            UNITS::CELSIUS => CELSIUS::from(unit).0,
            UNITS::FAHRENHEIT => FAHRENHEIT::from(unit).0,
        };
        println!("{}", converted_number);
        return;
        // */
    }
    fail();
}
