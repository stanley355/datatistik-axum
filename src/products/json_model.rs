use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Validate, Clone)]
pub(super) struct ProductLocalization {
    en_label: String,
    id_label: String,
    cn_label: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
struct ProductOptionValue {
    #[validate(length(min = 1, message = "id_values cannot be empty"))]
    id_values: String,

    #[validate(length(min = 1, message = "en_values cannot be empty"))]
    en_values: String,

    #[validate(length(min = 1, message = "cn_values cannot be empty"))]
    cn_values: String,

    #[validate(range(min = 0, message = "Price addition cannot be negative"))]
    price_addition: i64,

    #[validate(url(message = "Invalid image URL format"))]
    image_url: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub(super) struct ProductOption {
    #[validate(length(min = 1, message = "id_label cannot be empty"))]
    id_label: String,

    #[validate(length(min = 1, message = "en_label cannot be empty"))]
    en_label: String,

    #[validate(length(min = 1, message = "cn_label cannot be empty"))]
    cn_label: String,

    #[validate(nested, length(min = 1, message = "At least one value is required"))]
    values: Vec<ProductOptionValue>,
}
