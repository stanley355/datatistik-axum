use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::products::ProductOptionValue;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CartProductOption {
    #[validate(length(min = 1, message = "id_label cannot be empty"))]
    id: String,

    #[validate(length(min = 1, message = "en_label cannot be empty"))]
    en: String,

    #[validate(length(min = 1, message = "cn_label cannot be empty"))]
    cn: String,

    value: ProductOptionValue,
}
