use serde::{Deserialize, Serialize};
use validator::Validate;

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

impl ProductOption {
    pub(super) fn format_values_price(self) -> Self {
        let new_values: Vec<ProductOptionValue> = self
            .values
            .iter()
            .map(|val| ProductOptionValue {
                cn: val.cn.clone(),
                id: val.id.clone(),
                en: val.en.clone(),
                price_addition: val.price_addition * 100,
            })
            .collect();

        ProductOption {
            cn: self.cn,
            id: self.id,
            en: self.en,
            values: new_values,
        }
    }
}
