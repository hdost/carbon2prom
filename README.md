# carbon2prom

The purpose of carbon2prom is to bring graphite compatible applications to be compatible with prometheus `remote_write`.


## TODO

### Graphite Receiver
* [] UDP? Do I care?
* [] Pickle Protocol?
* []
* []


### Config file
* [] Yaml file input
* [] Regex mapping
* []
* []

### Reliability
* [] File Backed Queue/WAL concept
* []
* []

### Remote Write
* [] Batch writing
* [] Backoff
* []

## Proto
* [] Remove gogoproto

## Developing

### Prerequisites

* Rust 1.50+

### Building

* There shouldn't be anything out of the ordinary: `cargo build`.

### Testing

`cargo test`

## Contributing

* File an Issue if there's something wrong.
* Submit a PR if you want to fix that something wrong.
** File an issue also if you think it needs it.

## Code Of Conduct

Because it's easier than writing my own I abide by the [Rust Language Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). Addendum is just email me at:
harrassement@hdost.com if there is a complaint.

## License

`SPDX-License-Identifier: Apache-2.0`

## Disclaimer

This project is not associated with any organization. It is a pet project, but if you're interested in tooling around with it feel free.
