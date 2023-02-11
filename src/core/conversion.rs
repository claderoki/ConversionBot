pub enum ConversionError {
    NoConversions,
}

pub enum MeasurementKind {
    Currency,
    Unit,
}


pub struct Measurement {
    pub symbol: String,
    pub code: String,
    pub rate: u64,
    pub name: String,
    pub kind: MeasurementKind,
}

pub struct Conversion {}

pub struct ConversionService {

}

pub struct ConversionContext<'a> {
    pub measurement: &'a Measurement,
    pub value: f64,
}

pub struct ConversionRequest<'a> {
    from: &'a Measurement,
    value: f64,
    to_list: Vec<&'a Measurement>,
}

impl ConversionService {
    pub fn search(&self, _content: String) -> Result<Vec<ConversionContext<'static>>, ConversionError> {
        Err(ConversionError::NoConversions)
    }

    pub fn convert(&self, _request: ConversionRequest) -> Result<Conversion, ConversionError> {
        Ok(Conversion {})
    }
}
