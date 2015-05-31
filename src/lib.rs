extern crate serde;

use std::iter;

pub mod encode;
pub mod decode;
pub mod format;
pub mod error;

pub use self::error::{
    Error,
    ErrorCode
};

pub use self::decode::{
    Decoder,
    from_iter,
    from_reader,
    from_slice,
    from_str,
};

pub use self::encode::{
    Encoder,
    to_writer,
    to_writer_pretty,
    to_vec,
    to_vec_pretty,
    to_string,
    to_string_pretty,
};
