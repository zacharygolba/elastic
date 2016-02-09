# elasticsearch-rs
Yet another work in progress Elasticsearch client for Rust.

Platform  | Status
------------- | -------------
Linux / OSX  | [![Build Status](https://travis-ci.org/KodrAus/elasticsearch-rs.svg?branch=master)](https://travis-ci.org/KodrAus/elasticsearch-rs)
Windows  | [![Build status](https://ci.appveyor.com/api/projects/status/s0yo6i7sr4kc5sd5?svg=true)](https://ci.appveyor.com/project/KodrAus/elasticsearch-rs)

Currently only compiling on the `nightly` channel. Support for `stable` will be added.

If the build is red, you can check the Travis build history to find the last `nightly` version that worked. Failures are usually because of changes to dependencies upstream.

## Roadmap

*Not useable yet*

See [milestones](https://github.com/KodrAus/elasticsearch-rs/milestones).

- Codegen API endpoints
- Implement core Elasticsearch types
- io layer
- Doc APIs
- Query DSL proof-of-concept to test design
- Everything else

## Goals

To provide a strongly-typed, full-featured and efficient Elasticsearch client for Rust over (eventually) asynchronous io.

The REST API will be provided by inline JSON so it's efficient and always in line with whatever version of Elasticsearch you're targeting. For example:

```
let colour = "blue";
let count = 5;
let result = search_index_type(
  "myindex", "mytype", 
    body!(colour, count, {
      nested : {
        path : "obj1",
        score_mode : "avg",
        query : {
          bool : {
            must : [
              {match : {obj1.name : $colour }},
              {range : {obj1.count : {gt : $count}}}
            ]
          }
        }
      }
    }
  )
);
```

## Design

The client is divided into a few crates by utility. These will probably be moved into their own repositories to tidy up build/test, but for now it's conventient to develop them together.

### elastic_codegen

[Docs](http://kodraus.github.io/rustdoc/elastic_codegen/)

Provides code generation for the Elasticsearch REST API from the official [spec](https://github.com/elastic/elasticsearch/tree/master/rest-api-spec) and generic helpers for rust source and integration tests. The goal is to keep this package fairly agnostic, so the same `ast` can be used to generate other kinds of output.

Right now, it's used be `elastic_hyper` to build the low-level client, but could also be used to generate other implementations, like `elastic_rotor`.

### elastic_hyper

Provides a [hyper]("https://github.com/hyperium/hyper") implementation of the Elasticsearch REST API. This is the 'low-level' client that works purely through JSON. This crate is responsible for the `gen` in `elastic_codegen` and builds its own source and tests.

### elastic_types

[Docs](http://kodraus.github.io/rustdoc/elastic_types/)

Provides rust implementations of the main Elasticsearch types (like `date`) and responses. Required by the high-level client and useful for the low-level client, especially if you're mostly just using the [Document API](https://www.elastic.co/guide/en/elasticsearch/reference/current/docs.html?q=document).

#### elastic_types_codegen

[Docs](http://kodraus.github.io/rustdoc/elastic_types_codegen/)

Provides compiler plugins and macros for the `elastic_types` crate, such as parsing a date format to an array of [Items](https://github.com/lifthrasiir/rust-chrono/blob/master/src/format/mod.rs#L161) at compile-time for efficient runtime date parsing.
