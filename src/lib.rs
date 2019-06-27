pub use crate::prost::*;

mod prost {
    include!("prost/tipb.rs");
    include!("prost/wrapper_tipb.rs");
}
