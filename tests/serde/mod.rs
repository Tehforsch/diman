#[cfg(test)]
#[cfg(feature = "f64")]
#[cfg(feature = "glam-dvec2")]
#[cfg(feature = "glam-dvec3")]
mod vector_tests {

    use crate::example_system::dvec2::Dimensionless as Vec2Dimensionless;
    use crate::example_system::dvec2::Length as Vec2Length;
    use crate::example_system::dvec3::Dimensionless as Vec3Dimensionless;
    use crate::example_system::dvec3::Length as Vec3Length;
    use crate::example_system::f64::Length;
    use crate::utils::assert_is_close;

    #[test]
    fn deserialize_vector_2() {
        let q: Vec2Length = serde_yaml::from_str("(5.0 3.0) km").unwrap();
        assert_is_close(q.x(), Length::kilometers(5.0));
        assert_is_close(q.y(), Length::kilometers(3.0));
    }

    #[test]
    fn deserialize_vector_3() {
        let q: Vec3Length = serde_yaml::from_str("(5.0 3.0 7.0) km").unwrap();
        assert_is_close(q.x(), Length::kilometers(5.0));
        assert_is_close(q.y(), Length::kilometers(3.0));
        assert_is_close(q.z(), Length::kilometers(7.0));
    }

    #[test]
    #[should_panic]
    fn deserialize_vector_2_fails_with_fewer_than_2_components() {
        let _: Vec2Length = serde_yaml::from_str("(5.0) km").unwrap();
    }

    #[test]
    #[should_panic]
    fn deserialize_vector_2_fails_with_more_than_2_components() {
        let _: Vec2Length = serde_yaml::from_str("(5.0 3.0 7.0) km").unwrap();
    }

    #[test]
    #[should_panic]
    fn deserialize_vector_3_fails_with_fewer_than_3_components() {
        let _: Vec3Length = serde_yaml::from_str("(5.0 4.0) km").unwrap();
    }

    #[test]
    #[should_panic]
    fn deserialize_vector_3_fails_with_more_than_3_components() {
        let _: Vec3Length = serde_yaml::from_str("(5.0 3.0 7.0 9.0) km").unwrap();
    }

    #[test]
    fn serialize_vector_2() {
        let x = Vec2Length::meters(5.3, 1.1);
        let result: String = serde_yaml::to_string(&x).unwrap();
        assert_eq!(result, "(5.3 1.1) m\n");
    }

    #[test]
    fn serialize_vector_3() {
        let x = Vec3Length::meters(5.3, 1.1, 2.2);
        let result: String = serde_yaml::to_string(&x).unwrap();
        assert_eq!(result, "(5.3 1.1 2.2) m\n");
    }

    #[test]
    fn serialize_dimensionless_vector_2() {
        let x = Vec2Dimensionless::dimensionless(5.3, 1.1);
        let result: String = serde_yaml::to_string(&x).unwrap();
        assert_eq!(result, "(5.3 1.1)\n");
    }

    #[test]
    fn serialize_dimensionless_vector_3() {
        let x = Vec3Dimensionless::dimensionless(5.3, 1.1, 2.2);
        let result: String = serde_yaml::to_string(&x).unwrap();
        assert_eq!(result, "(5.3 1.1 2.2)\n");
    }
}
