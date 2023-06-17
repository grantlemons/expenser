pub mod database;
pub mod models;
pub mod schema;

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

    pub mod invoice;
    pub mod invoice_line_item;
    pub mod invoice_permissions;
    pub mod invoice_proof;
    pub mod user;
}
