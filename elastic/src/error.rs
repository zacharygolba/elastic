//! Client-side error types.
//! 
//! The main `Error` type combines the various kinds of errors
//! that can occur when interacting with Elasticsearch.
//! These include:
//! 
//! - `Api`: an error directly from an Elasticsearch exception,
//! like `index_not_found`.
//! This is the probably the variant you'll be interested in handling.
//! - `Json`: a general error serialising or deserialising json.
//! - `Http`: a general error in the http transport.
//! 
//! # Examples
//! 
//! Any method defined in `elastic` that could fail will return a
//! `Result<T, Error>` that can be matched on.
//! The below example sends a request and then checks the response
//! for an `ErrorKind::Api`:
//! 
//! ```no_run
//! # use elastic::prelude::*;
//! # use elastic::error::*;
//! # let client = Client::new(RequestParams::default()).unwrap();
//! # let req = PingRequest::new();
//! // Send a request.
//! // This will return a Result<ResponseBuilder, Error>
//! let res = client.request(req)
//!                 .send();
//! 
//! match res {
//!     Ok(response) => {
//!         // do something with the response
//!     },
//!     Err(e) => {
//!         match *e.kind() {
//!             ErrorKind::Api(ref e) => {
//!                 // handle a REST API error
//!             },
//!             ref e => {
//!                 // handle a HTTP or JSON error
//!             }
//!         }
//!     }
//! }
//! ```

#![allow(missing_docs)]

pub use serde_json::Error as JsonError;
pub use reqwest::Error as HttpError;
pub use elastic_responses::error::{ApiError, ResponseError};

error_chain! {
    foreign_links {
        Api(ApiError);
        Json(JsonError);
        Http(HttpError);
    }
}

impl From<ResponseError> for Error {
    fn from(err: ResponseError) -> Self {
        match err {
            ResponseError::Api(err) => ErrorKind::Api(err).into(),
            ResponseError::Json(err) => ErrorKind::Json(err).into(),
        }
    }
}
