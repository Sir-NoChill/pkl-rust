//! # PKL
//!
//! [pkl](https://pkl-lang.org/) is Apple's open source Configuration-as-Code language
//! designed to build programmable, scalable and safe configurations
//! for large projects.
//!
//! This is a rust binding for the pkl language based on the
//! [Go implementation](https://github.com/apple/pkl-go.git) currently
//! in HEAVY DEVELOPMENT. This crate is not yet ready for use
//!
//! ## Design
//!
//! We define the message passing system used to communicate with the
//! pkl server using the [rmp_serde](https://docs.rs/rmp-serde/latest/rmp_serde/)
//! library, for which we thank the developers, to communicate with a
//! spawned pkl server in a subprocess. Communication with the server
//! occurs through message passing, however there are a few bugs to work out...
//!
//! ## Development Progress
//!
//! Currently the development process is ongoing, please see the
//! [github repo](https://github.com/sir-nochill/pkl-rust) for a complete listing
//! of all currently in progress features.
//!
//! ## Feature List
//!
//! We aim to conform to the pkl-lang specification for a language binding.
//! The workflow will be as follows (once the crate is complete):
//! - Write your pkl file
//!   - AKA: your file that users should 'amend
//! - Generate the binding module from your .pkl file
//! - `use pkl` to get your configuration data
//!
//! Currently, the generation of the binding module is in the works,
//! however carefully crafting both your .pkl file and your rust data structure
//! will yeild the correct results. The target project structure is as follows:
//! ```norun
//! ProjectDir
//! ├── PklDir
//! └── src
//!      ├── pkl
//!      │    └── // generated sources
//!      ├── main.rs
//!      └── pkl.rs
//! ```
//! from which you can import the pkl structures

///////////////////////////////////////////////////////////////////////////

#![cfg_attr(feature = "unstable", feature(*))]
pub mod evaluator;
