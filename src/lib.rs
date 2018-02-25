#![feature(conservative_impl_trait)]
#![feature(non_exhaustive)]
#![feature(slice_patterns)]
#![feature(try_from)]
#![feature(universal_impl_trait)]

extern crate ascii;
extern crate bytes;
extern crate chrono;
extern crate fnv;
#[macro_use]
extern crate futures;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate tokio;
extern crate tokio_io;
extern crate tokio_service;
extern crate url;

mod syntax;

pub mod client;
pub mod header;
pub mod method;
pub mod protocol;
pub mod reason;
pub mod request;
pub mod response;
pub mod session;
pub mod status;
pub mod uri;
pub mod version;

pub use header::{HeaderMap, HeaderName, HeaderValue, TypedHeader, TypedHeaderMap};
pub use method::Method;
pub use reason::ReasonPhrase;
pub use request::Request;
pub use response::Response;
pub use session::SessionID;
pub use status::{StatusCode, StatusCodeClass};
pub use uri::URI;
pub use version::Version;
