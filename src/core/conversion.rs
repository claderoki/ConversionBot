pub enum ConversionError {
    NoConversions,
}

pub struct Measurement {}

pub struct Conversion {}

pub struct ConversionService {

}

pub struct ConversionContext<'a> {
    measurement: &'a Measurement,
    value: f64,
}

pub struct ConversionRequest<'a> {
    from: &'a Measurement,
    value: f64,
    to_list: Vec<&'a Measurement>,
}

impl ConversionService {
    pub fn match_() -> Result<Vec<ConversionContext>, ConversionError> {
        Err(ConversionError::NoConversions)
    }


    pub fn convert(&self, request: ConversionRequest) -> Result<Conversion, ConversionError> {

        Ok(Conversion {})
    }
}
