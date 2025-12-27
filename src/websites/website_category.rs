use crate::schema::sql_types;
use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Debug, AsExpression, Deserialize, Serialize, Clone, FromSqlRow)]
#[diesel(sql_type = sql_types::WebsiteCategory)]
pub enum WebsiteCategory {
    Government,
    Economy,
    Statistics,
    Tourism,
}

impl ToSql<sql_types::WebsiteCategory, Pg> for WebsiteCategory {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            WebsiteCategory::Government => out.write_all(b"GOVERNMENT")?,
            WebsiteCategory::Economy => out.write_all(b"ECONOMY")?,
            WebsiteCategory::Statistics => out.write_all(b"STATISTICS")?,
            WebsiteCategory::Tourism => out.write_all(b"TOURISM")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<sql_types::WebsiteCategory, Pg> for WebsiteCategory {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"GOVERNMENT" => Ok(WebsiteCategory::Government),
            b"ECONOMY" => Ok(WebsiteCategory::Economy),
            b"STATISTICS" => Ok(WebsiteCategory::Statistics),
            b"TOURISM" => Ok(WebsiteCategory::Tourism),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
