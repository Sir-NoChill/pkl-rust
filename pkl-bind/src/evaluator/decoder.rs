
/// Derive trait for unmarshalling data from the PKL
/// binary format as described
/// [here](https://pkl-lang.org/main/current/bindings-specification/binary-encoding.html).
///
/// We provide a default implementation using serde
/// and rmp_serde that handles basic datatypes and
/// pkl inheritance in an idiomatic rust style.
pub trait Pkl {
    fn unmarshal(data: Vec<u8>) -> Result<Self, &'static str> where Self: Sized;
}

#[cfg(test)]
mod tests {
    use pkl_derive::Pkl;

    use super::*;

    #[test]
    fn test_simple_decode() {
        #[derive(Debug, Pkl)]
        struct Test {
            foo: i64,
            bar: i32,
        }

        // Test {foo: 1, bar: 2}
        let data = vec![0x94, 0x01, 0xA4, 0x54, 0x65, 0x73, 0x74, 0xD9, 0x44, 0x66, 0x69, 0x6C, 0x65,
                        0x3A, 0x2F, 0x2F, 0x2F, 0x68, 0x6F, 0x6D, 0x65, 0x2F, 0x73, 0x74, 0x6F, 0x72,
                        0x6D, 0x62, 0x6C, 0x65, 0x73, 0x73, 0x65, 0x64, 0x2F, 0x43, 0x6F, 0x64, 0x65,
                        0x2F, 0x70, 0x6B, 0x6C, 0x2D, 0x72, 0x75, 0x73, 0x74, 0x2F, 0x73, 0x72, 0x63,
                        0x2F, 0x65, 0x76, 0x61, 0x6C, 0x75, 0x61, 0x74, 0x6F, 0x72, 0x2F, 0x74, 0x65,
                        0x73, 0x74, 0x73, 0x2F, 0x74, 0x65, 0x73, 0x74, 0x2E, 0x70, 0x6B, 0x6C, 0x92,
                        0x93, 0x10, 0xA3, 0x66, 0x6F, 0x6F, 0x01, 0x93, 0x10, 0xA3, 0x62, 0x61, 0x72,
                        0x02];

        let test = Test::unmarshal(data).unwrap();
        assert_eq!(test.foo, 1);
        assert_eq!(test.bar, 2);
        println!("Unmarshalled: {:?}", test);
    }
}
