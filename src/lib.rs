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

///////////////////////////////////////////////////////////////////////////

#![cfg_attr(feature = "unstable", feature(*))]
pub mod generator;
pub mod evaluator;
