#[cfg(test)]
mod uom_tests {
    use uom::si::length::meter;

    #[test]
    fn test_uom_and_num_units_comparison() {
        // UOM example
        let uom_distance = uom::si::f64::Length::new::<meter>(100.0);

        // num-units example
        let num_units_distance = num_units::si::length::Length::from_base(100.0);

        // Both represent 100 meters
        assert_eq!(uom_distance.get::<uom::si::length::meter>(), 100.0);
        assert_eq!(
            num_units_distance.to::<num_units::si::length::Meter>(),
            100.0
        );
    }
}
