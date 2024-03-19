use std::{any::Any, collections::HashMap};

// FIXME we cannot use the standard std::Any type,
//  we may need to implement a union of the real types
//  available in pkl.

/// The Rust representation of pkl.base#Object
/// Container for properties, entries and elements
struct Object {
    /// The URI of the module holding the definition
    uri: String,

    /// The qualified name of the pkl object class
    /// ex) pkl.base#Dynamic
    name: String,

    // /// The set of any name-value pairs in an object
    // properties: HashMap<String, dyn Any>,

    // /// The set of key-value pairs in an object
    // entries: HashMap<dyn Any, dyn Any>,

    // /// the set of items in an object
    // elements: Vec<dyn Any>,
}

/// The Regex type in pkl.base#Regex
struct Regex {
    pattern: String,
}

/// Compatibility for the pkl.base#Class
struct Class {}

/// Compatibility for the pkl.base#TypeAlias
struct TypeAlias {}

///  A struct for pkl compatibility with Duration type
struct Duration {
    /// The value of the duration in unit
    value: f64,

    /// Duration encoded from an enum
    unit: DurationUnit,
}

enum DurationUnit {
    Nanosecond,
    Microsecond,
    Millisecond,
    Second,
    Minute,
    Hour,
    Day,
}

fn rust_duration(duration: Duration) -> std::time::Duration {
    let dur: u64 = match duration.unit {
        DurationUnit::Nanosecond => (duration.value) as u64,
        DurationUnit::Microsecond => (duration.value * 1000_f64) as u64,
        DurationUnit::Millisecond => (duration.value * 10_f64.powf(6.)) as u64,
        DurationUnit::Second => (duration.value * 10_f64.powf(9.)) as u64,
        DurationUnit::Minute => (duration.value * 10_f64.powf(9.) * 60_f64) as u64,
        DurationUnit::Hour => (duration.value * 10_f64.powf(9.) * 60_f64.powf(2.)) as u64,
        DurationUnit::Day => (duration.value * 10_f64.powf(9.) * 60_f64.powf(2.) * 24_f64) as u64,
    };

    return std::time::Duration::from_nanos(dur);
}
