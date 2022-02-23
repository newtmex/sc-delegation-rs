pub mod bls_key;
pub mod bls_sig;
pub mod node_state;

pub use bls_key::BLSKey;
pub use bls_sig::BLSSignature;
use elrond_wasm::elrond_codec::multi_types::MultiValue2;
pub use node_state::*;

pub type BLSStatusMultiArg<M> = MultiValue2<BLSKey<M>, i32>;
