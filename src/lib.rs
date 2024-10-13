pub mod controller;
pub mod crd;
pub mod error;
pub mod opts;
pub mod util;

macro_rules! app_id {
    () => {
        "rustcloak.k8s.eboland.de"
    };
    ($name:tt) => {
        concat!(app_id!(), "/", $name)
    };
}

pub(crate) use app_id;
