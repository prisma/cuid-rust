use crate::error::CuidError;
use crate::text::to_base_string;
use js_sys::Date;

pub fn timestamp() -> Result<String, CuidError> {
    to_base_string(Date::now().round() as u128)
}
