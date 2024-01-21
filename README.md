# insim.rs

A collection of crates to assist working with the [Live For Speed](https://lfs.net/)
racing simulator and it's [Insim](https://en.lfsmanual.net/wiki/InSim.txt) (protocol).

The intention is to provide a strongly typed, native rust implementation, rather
than a thin layer over a series of bytes and primitive types.

:warning: The API is not yet stable at this time. Use at your own risk. I am
currently not touching the version number.

You can track the path to stable by monitoring these issues: <https://github.com/theangryangel/insim.rs/issues?q=is%3Aissue+is%3Aopen+label%3A%22target%3A+1.0.0%22>.

If you're not sure where to start, you probably want to look at the README and examples within the `insim` crate.

| Crate        | Usage                                           |
| ------------ | ----------------------------------------------- |
| `insim`      | Connection and protocol implementation.         |
| `insim_core` | Contains core types shared across other crates. |
| `insim_pth`  | Implements a PTH file read/writer.              |
| `insim_smx`  | Implements a SMX file reader/writer.            |

If you're looking for race_directord that has been moved to <https://github.com/theangryangel/race_directord/>.

## Documentation

Until we're released, either:

- Please run `cargo doc --no-deps --open`
- Take a look at the examples in each crate

## TODO

- `git grep '\(TODO\|FIXME\|XXX\)'`
- check out the issues list
