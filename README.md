# How to install

Add the following line in your `Cargo.toml` under the `[dependencies]` section:

``` rust
science_kit = { git = "https://github.com/GraineDePomme/science_kit.git" }
```

# How to use

## Physical constants

Physical constants can be imported using the `constants` module:

``` rust
use science_kit::constants;

fn main() {
    println!("The avogadro constant is {} mol⁻¹", constants::AVOGADRO_CONSTANT);
}
```

The following constants are currently defined:

- `ATOMIC_MASS_CONSTANT` : The atomic mass constant in kg
- `AVOGADRO_CONSTANT` : The Avogadro constant in mol⁻¹
- `BOLTZMANN_CONSTANT` : The Boltzmann constant in J.K⁻¹
- `CONDUCTANCE_QUANTUM` : The quantum of conductance in S
- `ELECTRON_MASS` : The mass of an electron in kg
- `ELECTRON_VOLT` : The amount of energy of 1eV in J
- `ELEMENTARY_CHARGE` : The elementary charge in C
- `FARADAY_CONSTANT` : The Faraday constant in C.mol⁻¹
- `FINE_STRUCTURE_CONSTANT` : The fine-structure constant
- `FINE_STRUCTURE_CONSTANT_INVERSE` : The inverse of the fine-structure constant
- `JOSEPHSON_CONSTANT` : The Josephson constant in Hz.V⁻¹
- `MAGNETIC_FLUX_QUANTUM` : The quantum of magnetic flux in Wb
- `MOLAR_GAS_CONSTANT` : The molar gas constant in J.mol⁻¹.K⁻¹
- `NEWTONIAN_GRAVITATIONAL_CONSTANT` : The newtonian constant of gravitation in m³.kg⁻¹.s⁻²
- `PLANCK_CONSTANT` : The Planck constant in J.Hz⁻¹
- `PROTON_MASS` : The mass of a proton in kg
- `PROTON_ELECTRON_MASS_RATIO` : The ratio proton_mass / electron_mass
- `REDUCED_PLANCK_CONSTANT` : The reduced Planck constant in J.Hz⁻¹
- `RYDBERG_CONSTANT` : The Rydberg constant in m⁻¹
- `RYDBERG_CONSTANT_TIMES_C` : The Rydberg constant times c in Hz
- `STEFAN_BOLTZMANN_CONSTANT` : The Stefan-Boltzmann constant in W.m⁻².K⁴
- `VACUUM_ELECTRIC_PERMITTIVITY` : The vacuum electric permittivity in F.m⁻¹
- `VACUUM_MAGNETIC_PERMEABILITY` : The vacuum magnetic permeability in N.A⁻²
- `VON_KLITZING_CONSTANT` : The Von Klitzing constant in Ω
- `STANDARD_GRAVITY_ACCELERATION` : The standard acceleration of gravity in m.s⁻²
- `SPEED_OF_LIGHT` : The speed of light in vacuum in m.s⁻¹


## unit conversion

Unit conversion can be performed using the `units` module:

```rust
use science_kit::constants::{RYDBERG_CONSTANT};
use science_kit::units;

fn main() {
    let rydberg_m: f64 = RYDBERG_CONSTANT;
    let rydberg_nm: f64 = units::convert_length(RYDBERG_CONSTANT, units::Length::Meter, -1.0, units::Length::Nanometer);

    println!("The Rydberg constant is {} m⁻¹ or {} nm⁻¹", rydberg_m, rydberg_nm);
}
```

The conversion functions take (in this order) the value itself, the old unit, the exponent of the unit, and the new unit.

The following units can be used:

### `units::Time`

- `Picosecond`
- `Nanosecond`
- `Microsecond`
- `Millisecond`
- `Second`
- `Kilosecond`
- `Minute`
- `Hour`
- `Day`
- `SideralDay`
- `Week`
- `Year`
- `TropicalYear`
- `SideralYear`

### `units::Length`

- `Kilometer`
- `Meter`
- `Centimeter`
- `Millimeter`
- `Micrometer`
- `Nanometer`
- `Angstrom`
- `Miles`
- `Yard`
- `Feet`
- `Inch`
- `NauticalMiles`


### `units::Energy`

- `Joule`
- `Kilojoule`
- `GramCalorie`
- `Kilocalorie`
- `WattHour`
- `KilowattHour`
- `Electronvolt`
- `BritishThermalUnit`
- `USTherm`
- `FootPound`


### `units:::Mass`

- `Tonne`
- `Kilogram`
- `Gram`
- `Milligram`
- `Microgram`
- `ImperialTon`
- `USTon`
- `Stone`
- `Pound`
- `Ounce`


### `units::Angles`

- `Turn`
- `Arcsecond`
- `Arcminute`
- `Degree`
- `Gradian`
- `Radian`
- `Milliradian`


### `units::Temeperatures`

- `Celsius`
- `Farenheit`
- `Kelvin`


## The `Measure` data type

This library introduces a `Measure` data type, which is composed of a value and its uncertainty:

``` rust
pub struct Measure {
    pub value: f64,
    pub error: f64
}
```

All the basic arithmetic operations are defined and the uncertainty spreads accordingly.

``` rust
use science_kit::measure::*;

fn main() {
    let a = Measure { value: 3.2, error: 0.2 };
    let b = Measure { value: 8.1, error: 1.2 };
    
    println!("a = {}", a);          // a = 3.2 ± 0.2
    println!("b = {}", b);          // b = 8.1 ± 1.2
    println!("a / b = {}", a/b);    // a / b = 0.39506172839506176 ± 0.06352283488545864
}
```

