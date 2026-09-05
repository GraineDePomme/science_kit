use crate::units::Angle::*;
use crate::units::Energy::*;
use crate::units::Length::*;
use crate::units::Mass::*;
use crate::units::Temperature::*;
use crate::units::Time::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Time {
    Picosecond,
    Nanosecond,
    Microsecond,
    Millisecond,
    Second,
    Kilosecond,
    Minute,
    Hour,
    Day,
    SideralDay,
    Week,
    Year,
    TropicalYear,
    SideralYear,
}

pub fn convert_time(value: f64, from: Time, exponent: f64, to: Time) -> f64 {
    // We first convert the value in seconds
    let value_in_second = match from {
        Picosecond => value * (1.0e-12_f64).powf(exponent),
        Nanosecond => value * (1.0e-9_f64).powf(exponent),
        Microsecond => value * (1.0e-6_f64).powf(exponent),
        Millisecond => value * (1.0e-3_f64).powf(exponent),
        Second => value,
        Kilosecond => value * (1.0e3_f64).powf(exponent),
        Minute => value * (60_f64).powf(exponent),
        Hour => value * (3600_f64).powf(exponent),
        Day => value * (86_400_f64).powf(exponent),
        SideralDay => value * (86_164.1_f64).powf(exponent),
        Week => value * (604_800_f64).powf(exponent),
        Year => value * (3.154e7_f64).powf(exponent),
        TropicalYear => value * (31_556_926.08_f64).powf(exponent),
        SideralYear => value * (31_556_925.129_6_f64).powf(exponent),
    };

    // Then we convert it into the unit we really want
    match to {
        Picosecond => value_in_second / (1.0e-12_f64).powf(exponent),
        Nanosecond => value_in_second / (1.0e-9_f64).powf(exponent),
        Microsecond => value_in_second / (1.0e-6_f64).powf(exponent),
        Millisecond => value_in_second / (1.0e-3_f64).powf(exponent),
        Second => value_in_second,
        Kilosecond => value_in_second / (1.0e3_f64).powf(exponent),
        Minute => value_in_second / (60_f64).powf(exponent),
        Hour => value_in_second / (3600_f64).powf(exponent),
        Day => value_in_second / (86_400_f64).powf(exponent),
        SideralDay => value_in_second / (86_164.1_f64).powf(exponent),
        Week => value_in_second / (604_800_f64).powf(exponent),
        Year => value_in_second / (3.154e7_f64).powf(exponent),
        TropicalYear => value_in_second / (31_556_926.08_f64).powf(exponent),
        SideralYear => value_in_second / (31_556_925.129_6_f64).powf(exponent),
    }
}

pub enum Length {
    Kilometer,
    Meter,
    Centimeter,
    Millimeter,
    Micrometer,
    Nanometer,
    Angstrom,
    Miles,
    Yard,
    Feet,
    Inch,
    NauticalMiles,
}

pub fn convert_length(value: f64, from: Length, exponent: f64, to: Length) -> f64 {
    // We first convert the value in meters
    let value_in_meters = match from {
        Kilometer => value * (1000_f64).powf(exponent),
        Meter => value,
        Centimeter => value * (0.01_f64).powf(exponent),
        Millimeter => value * (0.001_f64).powf(exponent),
        Micrometer => value * (1.0e-6_f64).powf(exponent),
        Nanometer => value * (1.0e-9_f64).powf(exponent),
        Angstrom => value * (1.0e-10_f64).powf(exponent),
        Miles => value * (1609.34_f64).powf(exponent),
        Yard => value * (0.9144_f64).powf(exponent),
        Feet => value * (0.3048_f64).powf(exponent),
        Inch => value * (0.0254_f64).powf(exponent),
        NauticalMiles => value * (1852_f64).powf(exponent),
    };

    // Then we convert it into the unit we really want
    match to {
        Kilometer => value_in_meters / (1000_f64).powf(exponent),
        Meter => value_in_meters,
        Centimeter => value_in_meters / (0.01_f64).powf(exponent),
        Millimeter => value_in_meters / (0.001_f64).powf(exponent),
        Micrometer => value_in_meters / (1.0e-6_f64).powf(exponent),
        Nanometer => value_in_meters / (1.0e-9_f64).powf(exponent),
        Angstrom => value_in_meters / (1.0e-10_f64).powf(exponent),
        Miles => value_in_meters / (1609.34_f64).powf(exponent),
        Yard => value_in_meters / (0.9144_f64).powf(exponent),
        Feet => value_in_meters / (0.3048_f64).powf(exponent),
        Inch => value_in_meters / (0.0254_f64).powf(exponent),
        NauticalMiles => value_in_meters / (1852_f64).powf(exponent),
    }
}

pub enum Energy {
    Joule,
    Kilojoule,
    GramCalorie,
    Kilocalorie,
    WattHour,
    KilowattHour,
    Electronvolt,
    BritishThermalUnit,
    USTherm,
    FootPound,
}

pub fn convert_energy(value: f64, from: Energy, exponent: f64, to: Energy) -> f64 {
    // We first convert the value in joule
    let value_in_joule = match from {
        Joule => value,
        Kilojoule => value * (1000_f64).powf(exponent),
        GramCalorie => value * (4.184_f64).powf(exponent),
        Kilocalorie => value * (4184_f64).powf(exponent),
        WattHour => value * (3600_f64).powf(exponent),
        KilowattHour => value * (3.6e6_f64).powf(exponent),
        Electronvolt => value * (1.6022e-19_f64).powf(exponent),
        BritishThermalUnit => value * (1055.06_f64).powf(exponent),
        USTherm => value * (1.055e8_f64).powf(exponent),
        FootPound => value * (1.35582_f64).powf(exponent),
    };

    // Then we convert it into the unit we really want
    match to {
        Joule => value_in_joule,
        Kilojoule => value_in_joule / (1000_f64).powf(exponent),
        GramCalorie => value_in_joule / (4.184_f64).powf(exponent),
        Kilocalorie => value_in_joule / (4184_f64).powf(exponent),
        WattHour => value_in_joule / (3600_f64).powf(exponent),
        KilowattHour => value_in_joule / (3.6e6_f64).powf(exponent),
        Electronvolt => value_in_joule / (1.6022e-19_f64).powf(exponent),
        BritishThermalUnit => value_in_joule / (1055.06_f64).powf(exponent),
        USTherm => value_in_joule / (1.055e8_f64).powf(exponent),
        FootPound => value_in_joule / (1.35582_f64).powf(exponent),
    }
}

pub enum Mass {
    Tonne,
    Kilogram,
    Gram,
    Milligram,
    Microgram,
    ImperialTon,
    USTon,
    Stone,
    Pound,
    Ounce,
}

pub fn convert_mass(value: f64, from: Mass, exponent: f64, to: Mass) -> f64 {
    // We first convert the value in grams
    let value_in_gram = match from {
        Tonne => value * (1.0e6_f64).powf(exponent),
        Kilogram => value * (1000_f64).powf(exponent),
        Gram => value,
        Milligram => value * (0.001_f64).powf(exponent),
        Microgram => value * (1.0e-6_f64).powf(exponent),
        ImperialTon => value * (1.016e6_f64).powf(exponent),
        USTon => value * (907_185_f64).powf(exponent),
        Stone => value * (6_350.295_021_585_f64).powf(exponent),
        Pound => value * (453.592_501_541_785_7_f64).powf(exponent),
        Ounce => value * (28.349_531_346_361_605_f64).powf(exponent),
    };

    // Then we convert it into the unit we really want
    match to {
        Tonne => value_in_gram / (1.0e6_f64).powf(exponent),
        Kilogram => value_in_gram / (1000_f64).powf(exponent),
        Gram => value_in_gram,
        Milligram => value_in_gram / (0.001_f64).powf(exponent),
        Microgram => value_in_gram / (1.0e-6_f64).powf(exponent),
        ImperialTon => value_in_gram / (1.016e6_f64).powf(exponent),
        USTon => value_in_gram / (907_185_f64).powf(exponent),
        Stone => value_in_gram / (6_350.295_021_585_f64).powf(exponent),
        Pound => value_in_gram / (453.592_501_541_785_7_f64).powf(exponent),
        Ounce => value_in_gram / (28.349_531_346_361_605_f64).powf(exponent),
    }
}

pub enum Angle {
    Turn,
    Arcsecond,
    Arcminute,
    Degree,
    Gradian,
    Radian,
    Milliradian,
}

pub fn convert_angle(value: f64, from: Angle, exponent: f64, to: Angle) -> f64 {
    // We first convert the value in turns
    let value_in_turns = match from {
        Turn => value,
        Arcsecond => value * (1_296_000_f64).powf(exponent),
        Arcminute => value * (std::f64::consts::TAU).powf(exponent),
        Degree => value * (360_f64).powf(exponent),
        Gradian => value * (400_f64).powf(exponent),
        Radian => value * (std::f64::consts::TAU).powf(exponent),
        Milliradian => value * (std::f64::consts::TAU * 1000.0).powf(exponent),
    };

    // Then we convert it into the unit we really want
    match to {
        Turn => value_in_turns,
        Arcsecond => value_in_turns / (1_296_000_f64).powf(exponent),
        Arcminute => value_in_turns / (std::f64::consts::TAU).powf(exponent),
        Degree => value_in_turns / (360_f64).powf(exponent),
        Gradian => value_in_turns / (400_f64).powf(exponent),
        Radian => value_in_turns / (std::f64::consts::TAU).powf(exponent),
        Milliradian => value_in_turns / (std::f64::consts::TAU * 1000.0).powf(exponent),
    }
}

pub enum Temperature {
    Celsius,
    Farenheit,
    Kelvin,
}

pub fn convert_temperature(value: f64, from: Temperature, exponent: f64, to: Temperature) -> f64 {
    // We first convert the value in kelvin
    let value_in_kelvin = match from {
        Celsius => value * (273.15_f64).powf(exponent),
        Farenheit => value * (255.372_f64).powf(exponent),
        Kelvin => value,
    };

    // Then we convert it into the unit we really want
    match to {
        Celsius => value_in_kelvin / (273.15_f64).powf(exponent),
        Farenheit => value_in_kelvin / (255.372_f64).powf(exponent),
        Kelvin => value_in_kelvin,
    }
}
