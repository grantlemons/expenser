pub mod database;
mod models;
mod schema;

pub use model_implementations::user::UserInfo;
pub use models::*;

mod model_implementations {
    use super::models::*;

    mod traits {
        pub trait Builder<T>: Default {
            type Output;

            fn build(&self) -> Option<T>;
        }

        pub trait HasBuilder<B: Builder<T>, T> {
            fn builder() -> B {
                B::default()
            }
        }
    }

    pub mod report;
    pub mod report_access;
    pub mod report_line_item;
    pub mod report_proof;
    pub mod user;
}
