use std::any::Any;
use async_trait::async_trait;
use rocket::{Build, Rocket};

mod error;
mod factory;

pub use error::Error;
pub use factory::Factory;

#[async_trait]
pub trait Service
where
    Self: Any + Send + Sync,
{
    async fn deploy(&self, factory: &mut dyn Factory) -> Deployment;
}

pub enum Deployment {
    Rocket(Rocket<Build>),
}

impl From<Rocket<Build>> for Deployment {
    fn from(r: Rocket<Build>) -> Self {
        Deployment::Rocket(r)
    }
}

#[macro_export]
macro_rules! declare_service {
    ($service_type:ty, $constructor:path) => {
        #[no_mangle]
        pub extern "C" fn _create_service() -> *mut dyn $crate::Service {
            // Ensure constructor returns concrete type.
            let constructor: fn() -> $service_type = $constructor;

            let obj = constructor();
            let boxed: Box<dyn $crate::Service> = Box::new(obj);
            Box::into_raw(boxed)
        }
    };
}
