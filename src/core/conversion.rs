use regex::Regex;
use itertools::Itertools;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub enum ConversionError {
    NoConversions,
}

#[derive(Debug)]
pub enum MeasurementKind {
    Currency,
    Unit,
}

impl MeasurementKind {
    pub fn is_currency(&self) -> bool {
        matches!(*self, MeasurementKind::Currency)
    }
}

#[derive(Debug)]
pub struct Measurement {
    pub symbol: String,
    pub code: String,
    pub rate: f64,
    pub name: String,
    pub kind: MeasurementKind,
}

#[derive(Debug)]
pub struct MeasurementConversion {
    pub to: Arc<Measurement>,
    pub value: f64,
}

#[derive(Debug)]
pub struct Conversion {
    pub from: Arc<Measurement>,
    pub base_value: f64,
    pub to: Vec<MeasurementConversion>,
}

#[derive(Debug)]
struct Regexes {
    currency_regex: Regex,
    measurement_regex: Regex,
}

impl Regexes {
    fn format_raw(regex: &str, values: Vec<String>) -> Regex {
        Regex::new(
            regex
                .replace("{values}", values.join("|").as_str())
                .as_str(),
        )
        .unwrap()
    }

    pub fn new(currencies: Vec<String>, units: Vec<String>) -> Self {
        Self {
            currency_regex: Self::format_raw(r"({values})(\d+(\.\d+)*)(?:$|\n|)", currencies),
            measurement_regex: Self::format_raw(r"([+-]?\d+(\.\d+)*)({values})(?:$|\n|)", units),
        }
    }
}

#[derive(Debug)]
pub struct ConversionContext {
    pub measurement: Arc<Measurement>,
    pub value: f64,
}

#[derive(Debug)]
pub struct ConversionRequest {
    pub from: Arc<Measurement>,
    pub value: f64,
    pub to_list: Vec<Arc<Measurement>>,
}

#[derive(Debug)]
pub struct ConversionService {
    measurements: Mutex<Vec<Arc<Measurement>>>,
    regexes: Regexes,
}

impl ConversionService {
    fn should_skip_currency(measurement: &Measurement) -> bool {
        matches!(measurement.code.to_lowercase().as_str(), "p" | "k" | "s" | "r" | "t" | "e" | "d" | "m" | "km" | "g" | "ar" | "l" | "le" | "ush" | "br")
            || matches!(measurement.symbol.to_lowercase().as_str(), "$")
    }

    pub fn new(measurements: Vec<Measurement>) -> Self {
        let currency_codes: Vec<String> = measurements
            .iter()
            .filter(|m| m.kind.is_currency() && !Self::should_skip_currency(m))
            .map(|m| m.symbol.to_lowercase())
            .dedup()
            .collect();
        let measurements_codes: Vec<String> = measurements
            .iter()
            .filter(|m| !m.kind.is_currency())
            .map(|m| m.code.to_lowercase())
            .dedup()
            .collect();

        Self {
            regexes: Regexes::new(currency_codes, measurements_codes),
            measurements: Mutex::new(measurements.into_iter().map(Arc::new).collect()),
        }
    }

    fn find_by<F: Fn(&Measurement) -> bool>(&self, func: F) -> Option<Arc<Measurement>> {
        if let Ok(measurements) = self.measurements.try_lock() {
            for measurement in measurements.iter() {
                if func(measurement) {
                    return Some(measurement.clone());
                }
            }
        }
        None
    }

    pub fn search(&self, content: &str) -> Result<Vec<ConversionContext>, ConversionError> {
        let currency_iter = self
            .regexes
            .currency_regex
            .captures_iter(content)
            .filter_map(|c| {
                Some((
                    c.get(1)?.as_str(),
                    c.get(2)?.as_str().parse::<f64>().ok()?,
                    MeasurementKind::Currency,
                ))
            });
        let measurement_iter = self
            .regexes
            .measurement_regex
            .captures_iter(content)
            .filter_map(|c| {
                Some((
                    c.get(1)?.as_str(),
                    c.get(2)?.as_str().parse::<f64>().ok()?,
                    MeasurementKind::Unit,
                ))
            });

        let contexts: Vec<ConversionContext> = currency_iter
            .chain(measurement_iter)
            .filter_map(|(unit, value, kind)| {
                let measurement = match kind {
                    MeasurementKind::Currency => self.find_by(|m| m.code == unit)?,
                    MeasurementKind::Unit => self.find_by(|m| m.symbol == unit)?,
                };
                Some(ConversionContext { measurement, value })
            })
            .collect();

        if !contexts.is_empty() {
            return Ok(contexts);
        }

        Err(ConversionError::NoConversions)
    }

    pub fn convert(&self, request: ConversionRequest) -> Result<Conversion, ConversionError> {
        let to_list = request
            .to_list
            .into_iter()
            .map(|m| {
                let converted = if request.from.code == "c" && m.code == "f" {
                    (request.value * 1.8) + 32.0
                } else if request.from.code == "f" && m.code == "c" {
                    (request.value - 32.0) / 1.8
                } else {
                    m.rate * request.value / (request.from.rate)
                };
                MeasurementConversion {
                    to: m,
                    value: converted,
                }
            })
            .collect();

        Ok(Conversion {
            from: request.from,
            base_value: request.value,
            to: to_list,
        })
    }
}
