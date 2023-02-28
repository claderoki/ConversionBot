#[cfg(test)]
mod test_mod {
    use crate::core::conversion::{ConversionRequest, ConversionService, Measurement};
    use crate::core::conversion::MeasurementKind::Unit;

    #[test]
    fn test_conversion() {
        let conversion_service = ConversionService::new(vec![
            Measurement {
                symbol: "m".into(),
                code: "m".into(),
                rate: 1.0,
                name: "meters".into(),
                kind: Unit,
            },
            Measurement {
                symbol: "km".into(),
                code: "km".into(),
                rate: 0.001,
                name: "kilometers".into(),
                kind: Unit,
            },
        ]);

        let mut converted_successfully = false;

        for context in conversion_service
            .search("50km hi")
            .expect("Matches expected.")
        {
            let request = ConversionRequest {
                from: context.measurement,
                value: context.value,
                to_list: vec![conversion_service.find_by(|m| m.code == *"m").unwrap()],
            };

            let conversion = conversion_service.convert(request).unwrap();

            for to in conversion.to {
                assert_eq!(to.value, 50000.0);
                converted_successfully = true;
            }
        }

        assert_eq!(converted_successfully, true);
    }
}
