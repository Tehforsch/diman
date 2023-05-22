#[cfg(test)]
#[cfg(all(
    feature = "glam",
    feature = "glam-dvec2",
    feature = "glam-dvec3",
    feature = "f64"
))]
mod tests {
    use crate::test_system::dvec3::Velocity as Vec3Velocity;
    use crate::test_system::f64::{Length, Time};
    use glam::DVec2;
    use glam::DVec3;

    use crate::test_utils::assert_is_close_f64 as assert_is_close;

    #[test]
    fn debug_vector_2() {
        assert_eq!(
            format!("{:?}", crate::test_system::dvec2::Length::meters(1.0, 5.0)),
            "[1, 5] m"
        );
        assert_eq!(
            format!(
                "{:?}",
                crate::test_system::dvec2::Length::kilometers(1.0, 5.0)
            ),
            "[1, 5] km"
        );
    }

    #[test]
    fn debug_vector_3() {
        assert_eq!(
            format!(
                "{:?}",
                crate::test_system::dvec3::Length::meters(1.0, 5.0, 6.0)
            ),
            "[1, 5, 6] m"
        );
    }

    #[test]
    fn mul_vec3() {
        let multiplied = DVec3::new(1.0, 2.0, 3.0) * Length::meters(5.0);
        assert_is_close(multiplied.x(), Length::meters(5.0));
        assert_is_close(multiplied.y(), Length::meters(10.0));
        assert_is_close(multiplied.z(), Length::meters(15.0));
        let multiplied = Length::meters(5.0) * DVec3::new(1.0, 2.0, 3.0);
        assert_is_close(multiplied.x(), Length::meters(5.0));
        assert_is_close(multiplied.y(), Length::meters(10.0));
        assert_is_close(multiplied.z(), Length::meters(15.0));
    }

    // #[test]
    // fn mul_assign_vec3() {
    //     let mut vec = Vec3Length::meters(1.0, 2.0, 3.0);
    //     vec *= 3.0;
    //     assert_is_close(vec.x(), Length::meters(3.0));
    //     assert_is_close(vec.y(), Length::meters(6.0));
    //     assert_is_close(vec.z(), Length::meters(9.0));
    // }

    // #[test]
    // fn div_assign_vec3() {
    //     let mut vec = Vec3Length::meters(1.0, 2.0, 3.0);
    //     vec /= 2.0;
    //     assert_is_close(vec.x(), Length::meters(0.5));
    //     assert_is_close(vec.y(), Length::meters(1.0));
    //     assert_is_close(vec.z(), Length::meters(1.5));
    // }

    #[test]
    fn mul_quantity_vec3() {
        let multiplied = Vec3Velocity::meters_per_second(1.0, 2.0, 3.0) * Time::seconds(5.0);
        assert_is_close(multiplied.x(), Length::meters(5.0));
        assert_is_close(multiplied.y(), Length::meters(10.0));
        assert_is_close(multiplied.z(), Length::meters(15.0));
        let multiplied = Time::seconds(5.0) * Vec3Velocity::meters_per_second(1.0, 2.0, 3.0);
        assert_is_close(multiplied.x(), Length::meters(5.0));
        assert_is_close(multiplied.y(), Length::meters(10.0));
        assert_is_close(multiplied.z(), Length::meters(15.0));
    }

    #[test]
    fn div_vec3() {
        let divided = DVec3::new(1.0, 2.0, 3.0) / Length::meters(0.2);
        let base = 1.0 / Length::meters(1.0);
        assert_is_close(divided.x(), 5.0 * base);
        assert_is_close(divided.y(), 10.0 * base);
        assert_is_close(divided.z(), 15.0 * base);
    }

    #[test]
    fn mul_vec2() {
        let multiplied = DVec2::new(1.0, 2.0) * Length::meters(5.0);
        assert_is_close(multiplied.x(), Length::meters(5.0));
        assert_is_close(multiplied.y(), Length::meters(10.0));
        let multiplied = Length::meters(5.0) * DVec2::new(1.0, 2.0);
        assert_is_close(multiplied.x(), Length::meters(5.0));
        assert_is_close(multiplied.y(), Length::meters(10.0));
    }

    // #[test]
    // fn mul_assign_vec2() {
    //     let mut vec = Vec2Length::meters(1.0, 2.0);
    //     vec *= 3.0;
    //     assert_is_close(vec.x(), Length::meters(3.0));
    //     assert_is_close(vec.y(), Length::meters(6.0));
    // }

    // #[test]
    // fn div_assign_vec2() {
    //     let mut vec = Vec2Length::meters(1.0, 2.0);
    //     vec /= 2.0;
    //     assert_is_close(vec.x(), Length::meters(0.5));
    //     assert_is_close(vec.y(), Length::meters(1.0));
    // }

    #[test]
    fn div_vec2() {
        let divided = DVec2::new(1.0, 2.0) / Length::meters(0.2);
        let base = 1.0 / Length::meters(1.0);
        assert_is_close(divided.x(), 5.0 * base);
        assert_is_close(divided.y(), 10.0 * base);
    }
}
