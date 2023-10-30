use unit_converter_lib::{units::length::{UNITS, UnitContainer, self}, converter};

pub fn show_interface() {
    clear_screen();
    let input_type = get_input_type();
    clear_screen();
    println!("Enter length in {}", input_type.to_string()); 
    let input_number = get_float();
    let start_type = UnitContainer::new(input_type, input_number);
    let return_type = get_input_type();
    let result = match return_type {
        UNITS::MM => length::MM::from(start_type).0,
        UNITS::CM => converter::length::to_cm(start_type).0,
        UNITS::M => converter::length::to_m(start_type).0,
        UNITS::KM => converter::length::to_km(start_type).0,
        UNITS::INCH => converter::length::to_inch(start_type).0,
        UNITS::MILE => converter::length::to_mile(start_type).0,
        UNITS::FOOT => converter::length::to_foot(start_type).0,
        UNITS::YARD => converter::length::to_yard(start_type).0,
    };
    println!("result: {}", result);
}

fn get_float() -> f64 {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let number = input.trim().parse::<f64>();
        clear_screen();
        match number {
            Ok(v) => {
                return v;
            },
            Err(e) => println!("please enter a valid number: {}", e),
        }
    }
}
fn get_input_type() -> UNITS {
    let mut counter = 0;
    for unit in UNITS::iterator() {
        println!("{}) {}", counter+1, unit.to_string());
        counter+=1;
    }
    let selected_type = get_number_input_in_range(1, UNITS::iterator().len());
    UNITS::get_type(selected_type-1)
}

fn get_number_input_in_range(start: u32, end: usize) -> u32 {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let number = input.trim().parse::<i32>();
        clear_screen();
        match number {
            Ok(v) => {
                if v >= start as i32 && v <= end as i32 { return v as u32; }
                println!("entered number should be in range of {start} to {end}");
            },
            Err(e) => println!("please enter a valid number: {} | {}", e, input),
        }
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H\n");
}
