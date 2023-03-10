#[cfg(test)]
mod test_mod {
    use crate::core::conversion::{ConversionRequest, ConversionService, Measurement};
    use crate::core::conversion::MeasurementKind::Unit;

    #[test]
    fn test_search() {
        let conversion_service = ConversionService::new(get_static_measurements());

        let context = conversion_service
            .search("50km hi")
            .expect("Matches expected.")
            .remove(0);

        assert_eq!(context.value, 50.0);
        assert_eq!(context.measurement.code, "km");
    }

    #[test]
    fn test_convert() {
        let conversion_service = ConversionService::new(get_static_measurements());

        let request = ConversionRequest {
            from: conversion_service.find_by(|m| &m.code == "km").unwrap(),
            value: 50.0,
            to_list: vec![conversion_service.find_by(|m| &m.code == "m").unwrap()],
        };

        let mut conversion = conversion_service.convert(request).unwrap();
        let to = conversion.to.remove(0);

        assert_eq!(to.value, 50000.0);
    }


    fn get_static_measurements() -> Vec<Measurement> {
        vec![
            Measurement {
                symbol: "m".into(),
                code: "m".into(),
                rate: 1.0,
                name: "meters".into(),
                kind: Unit,
            },
            Measurement {
                symbol: "mm".into(),
                code: "mm".into(),
                rate: 1000.0,
                name: "millimeters".into(),
                kind: Unit,
            },
            Measurement {
                symbol: "cm".into(),
                code: "cm".into(),
                rate: 100.0,
                name: "centimeters".into(),
                kind: Unit,
            },
            Measurement {
                symbol: "km".into(),
                code: "km".into(),
                rate: 0.001,
                name: "kilometers".into(),
                kind: Unit,
            },
            Measurement {
                symbol: "\"".into(),
                code: "inch".into(),
                rate: 39.37007874,
                name: "inches".into(),
                kind: Unit,
            },
            Measurement {
                symbol: "'".into(),
                code: "ft".into(),
                rate: 3.2808399,
                name: "feet".into(),
                kind: Unit,
            },
            Measurement {
                symbol: "yd".into(),
                code: "yd".into(),
                rate: 1.0936133,
                name: "yards".into(),
                kind: Unit,
            },
            Measurement {
                symbol: "??C".into(),
                code: "c".into(),
                rate: 0.0,
                name: "celsius".into(),
                kind: Unit,
            },
            Measurement {
                symbol: "??F".into(),
                code: "f".into(),
                rate: 0.0,
                name: "fahrenheit".into(),
                kind: Unit,
            },
            Measurement {
                symbol: "K".into(),
                code: "k".into(),
                rate: 0.0,
                name: "kelvin".into(),
                kind: Unit,
            },
            Measurement {
                symbol: "g".into(),
                code: "g".into(),
                rate: 1.0,
                name: "grams".into(),
                kind: Unit,
            },
            Measurement {
                symbol: "tonne".into(),
                code: "tonne".into(),
                rate: 1e-6,
                name: "tonne".into(),
                kind: Unit,
            },
            Measurement {
                symbol: "oz".into(),
                code: "oz".into(),
                rate: 0.03527399072294044,
                name: "ounces".into(),
                kind: Unit,
            },
            Measurement {
                symbol: "lb".into(),
                code: "lb".into(),
                rate: 0.0022046244201837776,
                name: "pounds".into(),
                kind: Unit,
            },
            Measurement {
                symbol: "stone".into(),
                code: "stone".into(),
                rate: 0.0001574731232746851,
                name: "stone".into(),
                kind: Unit,
            },
            Measurement {
                symbol: "mg".into(),
                code: "mg".into(),
                rate: 1000.0,
                name: "milogram".into(),
                kind: Unit,
            },
            Measurement {
                symbol: "kg".into(),
                code: "kg".into(),
                rate: 0.001,
                name: "kilogram".into(),
                kind: Unit,
            },
            Measurement {
                symbol: "mi".into(),
                code: "mi".into(),
                rate: 0.0006213712,
                name: "miles".into(),
                kind: Unit,
            },
        ]
    }
}
