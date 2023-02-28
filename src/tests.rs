#[cfg(test)]
mod test_mod {
    use crate::core::conversion::{ConversionRequest, ConversionService};
    use crate::data::get_static_measurements;

    #[test]
    fn test_search() {
        let conversion_service = ConversionService::new(get_static_measurements());

        let context = conversion_service.search("50km hi")
            .expect("Matches expected.")
            .remove(0);

        assert_eq!(context.value, 50.0);
        assert_eq!(context.measurement.code, "km");
    }

    #[test]
    fn test_convert() {
        let conversion_service = ConversionService::new(get_static_measurements());

        let request = ConversionRequest {
            from: conversion_service.find_by(|m|&m.code == "km").unwrap(),
            value: 50.0,
            to_list: vec![conversion_service.find_by(|m|&m.code == "m").unwrap()],
        };

        let mut conversion = conversion_service.convert(request).unwrap();
        let to = conversion.to.remove(0);

        assert_eq!(to.value, 50000.0);
    }
}
