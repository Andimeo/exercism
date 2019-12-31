// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s }
    }
}

pub trait Planet {
    const ORBID_PERIOD_IN_EARTH_YEARS: f64;
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / Earth::SECONDS_IN_YEAR / Self::ORBID_PERIOD_IN_EARTH_YEARS
    }
}

macro_rules! planets {
    ( $( $name:ident; $value:expr ),*) => { $( planet!($name; $value); )* }
}

macro_rules! planet {
    ($name:ident; $value:expr) => {
        pub struct $name;
        impl Planet for $name {
            const ORBID_PERIOD_IN_EARTH_YEARS: f64 = $value;
        }
    };
}

planets![
    Mercury; 0.2408467,
    Venus; 0.61519726,
    Earth; 1.0,
    Mars; 1.8808158,
    Jupiter; 11.862615,
    Saturn; 29.447498,
    Uranus; 84.016846,
    Neptune; 164.79132
];

impl Earth {
    const SECONDS_IN_YEAR: f64 = 31557600.0;
}
