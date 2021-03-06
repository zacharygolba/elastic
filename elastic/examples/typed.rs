//! A basic typed example.
//! 
//! NOTE: This sample expects you have a node running on `localhost:9200`.
//! 
//! This sample does the following:
//! 
//! - Check if a particular index exists
//! - Create the index if it doesn't
//! - Put the mapping for a document type
//! - Index a document
//! - Search the index and iterate over hits

#[macro_use]
extern crate json_str;
#[macro_use]
extern crate elastic_derive;
#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate elastic;

use elastic::error::*;
use elastic::prelude::*;

const INDEX: &'static str = "typed_sample_index";

#[derive(Debug, Serialize, Deserialize, ElasticType)]
struct MyType {
    id: i32,
    title: String,
    timestamp: Date<DefaultDateFormat>
}

fn main() {
    // A HTTP client and request parameters
    let client = Client::new(RequestParams::default()).unwrap();
    
    // Create a document to index
    let doc = MyType {
        id: 1,
        title: String::from("A title"),
        timestamp: Date::now()
    };

    // Check if the doc exists and index if it doesn't
    ensure_indexed(&client, doc);

    // Do a search request
    let res = search(&client);

    println!("{:?}", res);
}

fn ensure_indexed(client: &Client, doc: MyType) {
    let index = Index::from(INDEX);
    let ty = MyType::name();
    let id = doc.id.to_string();
    let req = GetRequest::for_index_ty_id(index, ty, id);

    let get_res = client.request(req)
                        .send()
                        .and_then(|res| res.response::<GetResponse<MyType>>());

    match get_res {
        // The doc was found: no need to index
        Ok(GetResponse { source: Some(doc), .. }) => {
            println!("document already indexed: {:?}", doc);
        },
        // The index exists, but the doc wasn't found: map and index
        Ok(_) => {
            println!("indexing doc");

            put_doc(client, doc);
        },
        // No index: create it, then map and index
        Err(Error(ErrorKind::Api(ApiError::IndexNotFound { .. }), _)) => {
            println!("creating index and doc");

            put_index(client);
            put_doc(client, doc);
        },
        // Something went wrong: panic
        Err(e) => panic!(e)
    }
}

fn put_index(client: &Client) {
    let req = IndicesCreateRequest::for_index(INDEX, empty_body());

    client.request(req).send().unwrap();

    let index = Index::from(INDEX);
    let mapping = MyType::mapping();
    let req = IndicesPutMappingRequest::try_for_mapping((index, mapping)).unwrap();

    client.request(req).send().unwrap();
}

fn put_doc(client: &Client, doc: MyType) {
    let index = Index::from(INDEX);
    let id = Id::from(doc.id.to_string());
    let req = IndexRequest::try_for_doc((index, id, &doc)).unwrap();

    client.request(req)
          .params(|params| params.url_param("refresh", true))
          .send()
          .unwrap();
}

fn search(client: &Client) -> SearchResponse<MyType> {
    let body = json_str!({
        query: {
            query_string: {
                query: "title"
            }
        }
    });

    let req = SearchRequest::for_index(INDEX, body);

    client.request(req).send()
                       .and_then(|res| res.response())
                       .unwrap()
}
