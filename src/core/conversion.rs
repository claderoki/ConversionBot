use std::sync::{Arc, Mutex};

pub enum ConversionError {
    NoConversions,
    MeasurementNotFound,
}

pub enum MeasurementKind {
    Currency,
    Unit,
}

impl MeasurementKind {
    pub fn is_currency(&self) -> bool {
        matches!(*self, MeasurementKind::Currency)
    }
}


pub struct Measurement {
    pub symbol: String,
    pub code: String,
    pub rate: f64,
    pub name: String,
    pub kind: MeasurementKind,
}

pub struct MeasurementConversion {
    pub to: Arc<Measurement>,
    pub value: f64,
}

pub struct Conversion {
    pub from: Arc<Measurement>,
    pub base_value: f64,
    pub to: Vec<MeasurementConversion>,
}

pub struct ConversionService {
    measurements: Mutex<Vec<Arc<Measurement>>>,
}

pub struct ConversionContext {
    pub measurement: Arc<Measurement>,
    pub value: f64,
}

pub struct ConversionRequest {
    pub from: Arc<Measurement>,
    pub value: f64,
    pub to_list: Vec<Arc<Measurement>>,
}

impl ConversionService {
    pub fn new(measurements: Vec<Measurement>) -> Self {
        Self {
            measurements: Mutex::new(measurements.into_iter().map(Arc::new).collect())
        }
    }

    fn find_by<F: Fn(&Measurement) -> bool>(&self, func: F) -> Option<Arc<Measurement>> {
        if let Ok(measurements) = self.measurements.try_lock() {
            for measurement in measurements.into_iter() {
                if func(&measurement) {
                    return Some(measurement.clone());
                }
            }
        }
        None
    }

    pub fn search(&self, _content: String) -> Result<Vec<ConversionContext>, ConversionError> {
        Err(ConversionError::NoConversions)
    }

    pub fn convert(&self, request: ConversionRequest) -> Result<Conversion, ConversionError> {
        let to_list = request.to_list.into_iter().map(|m|{
            let converted = if request.from.code == "c" && m.code == "f" {
                (request.value * 1.8) + 32.0
            } else if request.from.code == "f" && m.code == "c" {
                (request.value - 32.0) / 1.8
            } else {
                m.rate * request.value / (request.from.rate)
            };
            MeasurementConversion {
                to: m,
                value: converted
            }
        }).collect();

        Ok(Conversion {
            from: request.from,
            base_value: request.value,
            to:to_list
        })
    }
}
