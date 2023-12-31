# Unit Converter
## usage
to import this library use
```rust
use unit_converter_lib;
```
to read the stored value from a unit use `.0`
## examples
convert 5 meters into inches
```rust
// import types
use unit_converter_lib::units::length;
// create a variable that stores 5 meters
let meters: length::M = length::M(5.0);
// convert the 5 meters to inches
let converted_result = length::INCH::from(meters);
// load the float value from the unit
let converted_result_as_float = converted_result.0;
```

get unit by name and save it in a unit container, after that we convert it to meters
```rust
use unit_converter_lib::units::length;
// get the unit by its name
let unit_type: length::UNITS = length::UNITS::by_name("foot");
let value: f64 = 5.0;
// create a unit container with a specified unit and value
let unit_container = length::UnitContainer::new(unit_type, value);
// convert the unit
let converted_result_as_float = length::M::from(unit_container).0;
```

## how to create a unit
```
use unit_converter_lib::units;
let unit = units::<unit type>::<unit>(<value as f64>);
```
<unit type> = length|mass|volume|speed|temperature

## how to create a unit container
### version 1
```
use unit_converter_lib::units;
let unit_type = units::<unit type>::UNITS::<unit>;
let uc = units::<unit type>::UnitContainer::new(unit_type, <value>);
```
<unit type> = length|mass|volume|speed|temperature
### version 2
```
use unit_converter_lib::units;
let uc = units::<unit type>::UnitContainer::<unit>(units::<unit type>::<unit>(<value>));
```
<unit type> = length|mass|volume|speed|temperature

## source for conversion rates
`https://www.isa.org/getmedia/5be3daca-5c44-4d9e-bf1c-a4aa55cfb759/CCST-Conversions-document.pdf`
