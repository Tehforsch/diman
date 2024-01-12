#[cfg(test)]
mod tests {
    use crate::example_system::f64::{Area, Energy, Length};

    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", Length::meters(50.0)), "50 m");
        assert_eq!(format!("{:?}", Area::square_meters(50.0)), "50 m^2");
        // Unknown unit
        let x = Length::meters(1.0) * Energy::joules(50.0);
        assert_eq!(format!("{:?}", x), "50 m^3 s^-2 kg");
        assert_eq!(format!("{:?}", Energy::joules(50.0)), "50 J");
    }
}
