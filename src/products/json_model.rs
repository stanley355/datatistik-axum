use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::s3::S3Image;

#[derive(Deserialize, Serialize, Debug, Validate, Clone)]
pub(super) struct ProductLocalization {
    en: String,
    id: String,
    cn: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
struct ProductOptionValue {
    #[validate(length(min = 1, message = "id_values cannot be empty"))]
    id: String,

    #[validate(length(min = 1, message = "en_values cannot be empty"))]
    en: String,

    #[validate(length(min = 1, message = "cn_values cannot be empty"))]
    cn: String,

    #[validate(range(min = 0, message = "Price addition cannot be negative"))]
    price_addition: i64,

    image_url: Option<S3Image>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub(super) struct ProductOption {
    #[validate(length(min = 1, message = "id_label cannot be empty"))]
    id: String,

    #[validate(length(min = 1, message = "en_label cannot be empty"))]
    en: String,

    #[validate(length(min = 1, message = "cn_label cannot be empty"))]
    cn: String,

    #[validate(nested, length(min = 1, message = "At least one value is required"))]
    values: Vec<ProductOptionValue>,
}
