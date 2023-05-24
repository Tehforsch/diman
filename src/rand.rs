#[cfg(test)]
#[cfg(any(feature = "f64"))]
mod tests {
    use rand::Rng;

    use crate::test_system::f64::Length;

    #[test]
    fn test_random_quantity_generation() {
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let x = rng.gen_range(Length::meters(0.0)..Length::kilometers(1.0));
            assert!(Length::meters(0.0) <= x);
            assert!(x < Length::meters(1000.0));
        }
    }
}
