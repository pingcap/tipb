#[cfg(feature = "prost-codec")]
pub use crate::prost::*;
#[cfg(feature = "protobuf-codec")]
pub use crate::protobuf::*;

#[cfg(feature = "protobuf-codec")]
mod protobuf;
#[cfg(feature = "prost-codec")]
mod prost {
    include!("prost/tipb.rs");
    include!("prost/wrapper_tipb.rs");
}
