#[cfg(test)]
mod tests {
    use crate::example_system::units::{joules, meters, square_meters};

    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", 50.0 * meters), "50 m");
        assert_eq!(format!("{:?}", 50.0 * square_meters), "50 m^2");
        // Unknown unit
        let x = 50.0 * joules * meters;
        assert_eq!(format!("{:?}", x), "50 m^3 s^-2 kg");
        assert_eq!(format!("{:?}", 50.0 * joules), "50 J");
    }
}
