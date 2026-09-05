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
