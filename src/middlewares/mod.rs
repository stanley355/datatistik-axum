mod request;
mod response;

pub use request::BetterAuth;
pub use response::{AxumResponse, DEFAULT_PER_PAGE, DataPagination, JsonResponse, Pagination};
