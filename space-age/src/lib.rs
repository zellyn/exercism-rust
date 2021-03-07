use duplicate::duplicate_inline;

#[derive(Debug)]
pub struct Duration {
    years: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration {
            years: (s as f64) / 31557600.0,
        }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

duplicate_inline! {
    [
        planet  conversion;
        [ Mercury ] [ 0.2408467 ];
        [ Venus ]   [ 0.61519726 ];
        [ Earth ]   [ 1.0 ];
        [ Mars ]    [ 1.8808158 ];
        [ Jupiter ] [ 11.862615 ];
        [ Saturn ]  [ 29.447498 ];
        [ Uranus ]  [ 84.016846 ];
        [ Neptune ] [ 164.79132 ];
    ]

    pub struct planet;

    impl Planet for planet {
        fn years_during(d: &Duration) -> f64 {
            d.years / conversion
        }
    }
}
