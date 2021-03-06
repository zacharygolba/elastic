//! Glue traits for documents and requests.
//! 
//! The `TryForDoc` and `TryForMapping` traits are functionally equivalent
//! but are used to support different conversions.
//! They are implemented for tuples of a document and additional parameters,
//! this may be different in a future where indices only support a single
//! document type.

use serde_json;
use super::client::requests::{Index, Id, IndexRequest, IndicesPutMappingRequest};
use super::types::prelude::*;

use super::error::*;

/// A trait for converting a [document](../../types/document/index.html)
/// into a [request](endpoints/index.html).
pub trait TryForDoc<T, M>: Sized {
    /// Try convert a document into a request type.
    fn try_for_doc(doc: T) -> Result<Self>;
}

/// A trait for converting a [document mapping](../../types/document/trait.DocumentMapping.html) 
/// mapping into a [request](endpoints/index.html).
pub trait TryForMapping<M>
    where Self: Sized
{
    /// Try convert a document mapping into a request.
    fn try_for_mapping(mapping: M) -> Result<Self>;
}

impl<'a, 'b, T, M> TryForDoc<(Index<'a>, &'b T), M> for IndexRequest<'a, String>
    where T: DocumentType<M>,
          M: DocumentMapping
{
    fn try_for_doc((index, doc): (Index<'a>, &'b T)) -> Result<Self> {
        let ty = T::name();

        let doc = serde_json::to_string(&doc)?;

        Ok(Self::for_index_ty(index, ty, doc))
    }
}

impl<'a, 'b, T, M> TryForDoc<(Index<'a>, Id<'a>, &'b T), M> for IndexRequest<'a, String>
    where T: DocumentType<M>,
          M: DocumentMapping
{
    fn try_for_doc((index, id, doc): (Index<'a>, Id<'a>, &'b T)) -> Result<Self> {
        let ty = T::name();

        let doc = serde_json::to_string(&doc)?;

        Ok(Self::for_index_ty_id(index, ty, id, doc))
    }
}

impl<'a, M> TryForMapping<(Index<'a>, M)> for IndicesPutMappingRequest<'a, String>
    where M: DocumentMapping
{
    fn try_for_mapping((index, mapping): (Index<'a>, M)) -> Result<Self> {
        let mapping = serde_json::to_string(&Document::from(mapping))?;

        Ok(Self::for_index_ty(index, M::name(), mapping))
    }
}

impl<'a, 'b, T, M> TryForDoc<(Index<'a>, &'b T), M> for IndicesPutMappingRequest<'a, String>
    where T: DocumentType<M>,
          M: DocumentMapping
{
    fn try_for_doc((index, _): (Index<'a>, &'b T)) -> Result<Self> {
        Self::try_for_mapping((index, T::mapping()))
    }
}
