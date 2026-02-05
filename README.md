# rusty-bikes
Rust scripts pull Capital Bikeshare data <, and to serialize parquet files?>.

## Install Rust
Install `rustup` via Rust's [install script](https://rust-lang.org/tools/install/)

[Quickstart docs](https://doc.rust-lang.org/book/ch01-01-installation.html).

If offline docs are needed, run `rustup doc
`.

## Pull Capital Bikeshare data
DC Capital Bikeshare data [index](https://capitalbikeshare.com/system-data).

[DC Capital Bikeshare History](https://s3.amazonaws.com/capitalbikeshare-data/index.html).


### Making HTTP calls in Rust
Want to get some simple HTTP calls up in Rust. [`reqwest`]
(https://crates.io/crates/reqwest) seems fairly popular, and includes async features. For
this iteration, don't need async, but will work on a `blocking` implementation of this 
lib in order to get familiar w/ the API, and to implement async down the road if 
warrented
([ref](https://users.rust-lang.org/t/what-does-it-take-to-make-an-http-request/125980)).

Alternative is [`ureq`](https://github.com/algesten/ureq)

### Handling data structures in Rust
Using `serde` to parse semi-structured data from HTTP responses, and potentially for serializing
to Parquet. [This](https://stackoverflow.com/questions/37970355/read-xml-file-into-struct)
StackOverflow thread outlines and example dealing with XML data.

### Unzipping
Using the `zip` [library](https://docs.rs/zip/7.2.0/zip/index.html) to unzip the hitoric CSVs.
Package seems relatively heavy, and also unzipping via a file interface seems clunky, but don't
want to spend a huge ammount of time handling zips at this stage.

