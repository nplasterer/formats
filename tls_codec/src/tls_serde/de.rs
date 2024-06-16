use core::ops::{AddAssign, MulAssign, Neg};
use std::println;

use alloc::string::String;
use alloc::vec::Vec;
use serde::de::{
    self, DeserializeSeed, EnumAccess, IntoDeserializer, MapAccess, SeqAccess, VariantAccess,
    Visitor,
};
use serde::Deserialize;

use super::error::{Error, Result};

use crate::{Deserialize as _, DeserializeBytes as _, VLBytes};

pub struct Deserializer<'de> {
    // The input data and characters are truncated off
    // the beginning as data is parsed.
    input: &'de [u8],
}

impl<'de> Deserializer<'de> {
    // By convention, `Deserializer` constructors are named like `from_xyz`.
    // That way basic use cases are satisfied by something like
    // `serde_json::from_str(...)` while advanced use cases that require a
    // deserializer can make one with `serde_json::Deserializer::from_str(...)`.
    pub fn from_slice(input: &'de [u8]) -> Self {
        Deserializer { input }
    }
}

// By convention, the public API of a Serde deserializer is one or more
// `from_xyz` methods such as `from_str`, `from_bytes`, or `from_reader`
// depending on what Rust types the deserializer is able to consume as input.
//
// This basic deserializer supports only `from_slice`.
pub fn from_slice<'a, T>(s: &'a [u8]) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_slice(s);
    let t = T::deserialize(&mut deserializer)?;
    if deserializer.input.is_empty() {
        Ok(t)
    } else {
        Err(Error::TrailingCharacters)
    }
}

// // SERDE IS NOT A PARSING LIBRARY. This impl block defines a few basic parsing
// // functions from scratch. More complicated formats may wish to use a dedicated
// // parsing library to help implement their Serde deserializer.
// impl<'de> Deserializer<'de> {
//     // Look at the first character in the input without consuming it.
//     fn peek_char(&mut self) -> Result<char> {
//         self.input.chars().next().ok_or(Error::Eof)
//     }

//     // Consume the first character in the input.
//     fn next_char(&mut self) -> Result<char> {
//         let ch = self.peek_char()?;
//         self.input = &self.input[ch.len_utf8()..];
//         Ok(ch)
//     }

//     // Parse the JSON identifier `true` or `false`.
//     fn parse_bool(&mut self) -> Result<bool> {
//         if self.input.starts_with("true") {
//             self.input = &self.input["true".len()..];
//             Ok(true)
//         } else if self.input.starts_with("false") {
//             self.input = &self.input["false".len()..];
//             Ok(false)
//         } else {
//             Err(Error::ExpectedBoolean)
//         }
//     }

//     // Parse a group of decimal digits as an unsigned integer of type T.
//     //
//     // This implementation is a bit too lenient, for example `001` is not
//     // allowed in JSON. Also the various arithmetic operations can overflow and
//     // panic or return bogus data. But it is good enough for example code!
//     fn parse_unsigned<T>(&mut self) -> Result<T>
//     where
//         T: AddAssign<T> + MulAssign<T> + From<u8>,
//     {
//         let mut int = match self.next_char()? {
//             ch @ '0'..='9' => T::from(ch as u8 - b'0'),
//             _ => {
//                 return Err(Error::ExpectedInteger);
//             }
//         };
//         loop {
//             match self.input.chars().next() {
//                 Some(ch @ '0'..='9') => {
//                     self.input = &self.input[1..];
//                     int *= T::from(10);
//                     int += T::from(ch as u8 - b'0');
//                 }
//                 _ => {
//                     return Ok(int);
//                 }
//             }
//         }
//     }

//     // Parse a possible minus sign followed by a group of decimal digits as a
//     // signed integer of type T.
//     fn parse_signed<T>(&mut self) -> Result<T>
//     where
//         T: Neg<Output = T> + AddAssign<T> + MulAssign<T> + From<i8>,
//     {
//         // Optional minus sign, delegate to `parse_unsigned`, negate if negative.
//         unimplemented!()
//     }

//     // Parse a string until the next '"' character.
//     //
//     // Makes no attempt to handle escape sequences. What did you expect? This is
//     // example code!
//     fn parse_string(&mut self) -> Result<&'de str> {
//         if self.next_char()? != '"' {
//             return Err(Error::ExpectedString);
//         }
//         match self.input.find('"') {
//             Some(len) => {
//                 let s = &self.input[..len];
//                 self.input = &self.input[len + 1..];
//                 Ok(s)
//             }
//             None => Err(Error::Eof),
//         }
//     }
// }

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    // Look at the input data to decide what Serde data model type to
    // deserialize as.
    // This is not supported for TLS.
    // Formats that support `deserialize_any` are known as self-describing.
    fn deserialize_any<V>(self, _: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!("TLS is not self describing.")
    }

    // Uses the `parse_bool` parsing function defined above to read the JSON
    // identifier `true` or `false` from the input.
    //
    // Parsing refers to looking at the input and deciding that it contains the
    // JSON value `true` or `false`.
    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // We parse bools as u8's
        // XXX[syntax]: This is not in the TLS syntax.
        let b = self.input[0] != 0;
        self.input = &self.input[1..];
        visitor.visit_bool(b)
    }

    // The `parse_signed` function is generic over the integer type `T` so here
    // it is invoked with `T=i8`. The next 8 methods are similar.
    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = self.input[0];
        self.input = &self.input[1..];
        visitor.visit_u8(v)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = u16::tls_deserialize(&mut (&self.input[0..2] as &[u8])).unwrap();
        self.input = &self.input[2..];
        visitor.visit_u16(v)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = u32::tls_deserialize(&mut (&self.input[0..4] as &[u8])).unwrap();
        self.input = &self.input[4..];
        visitor.visit_u32(v)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = u64::tls_deserialize(&mut (&self.input[0..8] as &[u8])).unwrap();
        self.input = &self.input[8..];
        visitor.visit_u64(v)
    }

    // Float parsing is stupidly hard.
    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    // Float parsing is stupidly hard.
    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    // The `Serializer` implementation on the previous page serialized chars as
    // single-character strings so handle that representation here.
    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // Parse a string, check that it is one character, call `visit_char`.
        unimplemented!()
    }

    // Refer to the "Understanding deserializer lifetimes" page for information
    // about the three deserialization flavors of strings in Serde.
    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        println!("deserialise string | input: {:x?}", self.input);
        // This is a variable length encoded byte vector.
        let v = VLBytes::tls_deserialize(&mut self.input).expect("failed to deserialize");
        let s = String::from_utf8(v.into()).expect("failed to convert to string");
        visitor.visit_str(&s)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    // The `Serializer` implementation on the previous page serialized byte
    // arrays as JSON arrays of bytes. Handle that representation here.
    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    // An absent optional is represented as the JSON `null` and a present
    // optional is represented as just the contained value.
    //
    // As commented in `Serializer` implementation, this is a lossy
    // representation. For example the values `Some(())` and `None` both
    // serialize as just `null`. Unfortunately this is typically what people
    // expect when working with JSON. Other formats are encouraged to behave
    // more intelligently if possible.
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!();
        // if self.input.starts_with("null") {
        //     self.input = &self.input["null".len()..];
        //     visitor.visit_none()
        // } else {
        //     visitor.visit_some(self)
        // }
    }

    // In Serde, unit means an anonymous value containing no data.
    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!();
        // if self.input.starts_with("null") {
        //     self.input = &self.input["null".len()..];
        //     visitor.visit_unit()
        // } else {
        //     Err(Error::ExpectedNull)
        // }
    }

    // Unit struct means a named value containing no data.
    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    // As is done here, serializers are encouraged to treat newtype structs as
    // insignificant wrappers around the data they contain. That means not
    // parsing anything other than the contained value.
    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    // Deserialization of compound types like sequences and maps happens by
    // passing the visitor an "Access" object that gives it the ability to
    // iterate through the data contained in the sequence.
    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        println!("deserialize seq | input: {:x?}", self.input);

        // Sequences first have the number of elements.
        // XXX[syntax]: This only works for variable length encodings right now.
        let (length, _read) =
            crate::quic_vec::rw::read_length(&mut self.input).expect("failed to read length");
        let value = visitor.visit_seq(SeqParser::new(self, length))?;

        // println!(
        //     "deserialize seq | input: {:x?}\n\telements: {length}",
        //     self.input
        // );

        Ok(value)
    }

    // Tuples look just like sequences in JSON. Some formats may be able to
    // represent tuples more efficiently.
    //
    // As indicated by the length parameter, the `Deserialize` implementation
    // for a tuple in the Serde data model is required to know the length of the
    // tuple before even looking at the input data.
    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    // Tuple structs look just like sequences in JSON.
    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    // Much like `deserialize_seq` but calls the visitors `visit_map` method
    // with a `MapAccess` implementation, rather than the visitor's `visit_seq`
    // method with a `SeqAccess` implementation.
    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!();
        // // Parse the opening brace of the map.
        // if self.next_char()? == '{' {
        //     // Give the visitor access to each entry of the map.
        //     let value = visitor.visit_map(CommaSeparated::new(self))?;
        //     // Parse the closing brace of the map.
        //     if self.next_char()? == '}' {
        //         Ok(value)
        //     } else {
        //         Err(Error::ExpectedMapEnd)
        //     }
        // } else {
        //     Err(Error::ExpectedMap)
        // }
    }

    // Notice the `fields` parameter - a "struct" in the Serde data model means
    // that the `Deserialize` implementation is required to know what the fields
    // are before even looking at the input data. Any key-value pairing in which
    // the fields cannot be known ahead of time is probably a map.
    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // Structs are sequences of the members in TLS syntax.
        // But without size prefixes.
        println!("name: {_name} | fields: {_fields:?}");
        let value = visitor.visit_seq(StructParser::new(self))?;
        Ok(value)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        println!("name: {_name} | variants: {_variants:?}");
        println!("self.input: {:?}", self.input);
        let value = visitor.visit_enum(Enum::new(self))?;
        // println!("{:#?}", value);

        Ok(value)

        // if self.peek_char()? == '"' {
        //     // Visit a unit variant.
        //     visitor.visit_enum(self.parse_string()?.into_deserializer())
        // } else if self.next_char()? == '{' {
        //     // Visit a newtype variant, tuple variant, or struct variant.
        //     let value = visitor.visit_enum(Enum::new(self))?;
        //     // Parse the matching close brace.
        //     if self.next_char()? == '}' {
        //         Ok(value)
        //     } else {
        //         Err(Error::ExpectedMapEnd)
        //     }
        // } else {
        //     Err(Error::ExpectedEnum)
        // }
    }

    // An identifier in Serde is the type that identifies a field of a struct or
    // the variant of an enum. In JSON, struct fields and enum variants are
    // represented as strings. In other formats they may be represented as
    // numeric indices.
    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    // Like `deserialize_any` but indicates to the `Deserializer` that it makes
    // no difference which `Visitor` method is called because the data is
    // ignored.
    //
    // Some deserializers are able to implement this more efficiently than
    // `deserialize_any`, for example by rapidly skipping over matched
    // delimiters without paying close attention to the data in between.
    //
    // Some formats are not able to implement this at all. Formats that can
    // implement `deserialize_any` and `deserialize_ignored_any` are known as
    // self-describing.
    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

/// Reading sequences of values
/// This is for structs where we don't know the length
struct StructParser<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> StructParser<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        StructParser { de }
    }
}

// `SeqAccess` is provided to the `Visitor` to give it the ability to iterate
// through elements of the sequence.
impl<'de, 'a> SeqAccess<'de> for StructParser<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        // Check if there are no more elements.
        if self.de.input.is_empty() {
            // XXX: This is not correct. We need to track the size of things.
            return Ok(None);
        }
        // Deserialize an array element.
        seed.deserialize(&mut *self.de).map(Some)
    }
}

struct SeqParser<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    num_elments: usize,
}

impl<'a, 'de> SeqParser<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>, num_elments: usize) -> Self {
        SeqParser { de, num_elments }
    }
}

impl<'de, 'a> SeqAccess<'de> for SeqParser<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        println!("next_element_seed: {:?}", self.de.input);
        if self.num_elments > 0 {
            self.num_elments -= 1;
            Ok(Some(DeserializeSeed::deserialize(seed, &mut *self.de)?))
        } else {
            Ok(None)
        }
    }
}

struct Enum<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> Enum<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        Enum { de }
    }
}

// `EnumAccess` is provided to the `Visitor` to give it the ability to determine
// which variant of the enum is supposed to be deserialized.
//
// Note that all enum deserialization methods in Serde refer exclusively to the
// "externally tagged" enum representation.
impl<'de, 'a> EnumAccess<'de> for Enum<'a, 'de> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: DeserializeSeed<'de>,
    {
        println!("variant_seed: {:?}", self.de.input);
        todo!();
        // // The `deserialize_enum` method parsed a `{` character so we are
        // // currently inside of a map. The seed will be deserializing itself from
        // // the key of the map.
        // let val = seed.deserialize(&mut *self.de)?;
        // // Parse the colon separating map key from value.
        // if self.de.next_char()? == ':' {
        //     Ok((val, self))
        // } else {
        //     Err(Error::ExpectedMapColon)
        // }
    }
}

// `VariantAccess` is provided to the `Visitor` to give it the ability to see
// the content of the single variant that it decided to deserialize.
impl<'de, 'a> VariantAccess<'de> for Enum<'a, 'de> {
    type Error = Error;

    // If the `Visitor` expected this variant to be a unit variant, the input
    // should have been the plain string case handled in `deserialize_enum`.
    fn unit_variant(self) -> Result<()> {
        Err(Error::ExpectedString)
    }

    // Newtype variants are represented in JSON as `{ NAME: VALUE }` so
    // deserialize the value here.
    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: DeserializeSeed<'de>,
    {
        seed.deserialize(self.de)
    }

    // Tuple variants are represented in JSON as `{ NAME: [DATA...] }` so
    // deserialize the sequence of data here.
    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        de::Deserializer::deserialize_seq(self.de, visitor)
    }

    // Struct variants are represented in JSON as `{ NAME: { K: V, ... } }` so
    // deserialize the inner map here.
    fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        de::Deserializer::deserialize_map(self.de, visitor)
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_struct() {
        use crate::alloc::{borrow::ToOwned, string::String, vec::Vec};

        #[derive(Deserialize, PartialEq, Debug)]
        struct Test {
            int: u32,
            seq: Vec<String>,
        }

        let serialised = &[
            0, 0, 0, 1, // 1u32
            2, // vec with len 2
            1, 0x61, // vec with len 1 and utf8 value
            1, 0x62, // vec with len 1 and utf8 value
        ];
        let expected = Test {
            int: 1,
            seq: vec!["a".to_owned(), "b".to_owned()],
        };

        let deserialised = from_slice(serialised).expect("could not deserialize");
        assert_eq!(expected, deserialised);
    }

    #[test]
    fn test_nested_struct() {
        use crate::alloc::{borrow::ToOwned, string::String, vec::Vec};

        #[derive(Deserialize, PartialEq, Debug)]
        struct Other {
            a: u8,
            b: u16,
            c: u32,
            d: u64,
            bytes: Vec<u8>,
        }

        #[derive(Deserialize, PartialEq, Debug)]
        struct Test {
            int: u32,
            other: Other,
            seq: Vec<String>,
        }

        let serialised = &[
            0, 0, 0, 1, // 1u32
            // Start other
            9, // 9u8
            0, 8, // 8u16
            0, 0, 0, 3, //3u32
            0, 0, 0, 0, 0, 0, 0, 2, // 2u64
            3, 7, 8, 9, // vec[7, 8, 9]
            // Start seq
            2, // vec with len 2
            1, 0x61, // vec with len 1 and utf8 value
            1, 0x62, // vec with len 1 and utf8 value
        ];
        let expected = Test {
            int: 1,
            seq: vec!["a".to_owned(), "b".to_owned()],
            other: Other {
                a: 9,
                b: 8,
                c: 3,
                d: 2,
                bytes: vec![7, 8, 9],
            },
        };

        let deserialised = from_slice(serialised).expect("could not deserialize");
        assert_eq!(expected, deserialised);
    }

    #[test]
    fn test_enum() {
        #[derive(Deserialize, PartialEq, Debug)]
        struct Nested {
            int: u32,
            seq: Vec<u8>,
        }

        #[derive(Deserialize, PartialEq, Debug)]
        enum E {
            Unit,
            Newtype(u32),
            Tuple(u32, u32),
            Struct { a: u32 },
            Nested(Nested),
        }

        // unit
        let expected_e_unit = E::Unit;
        let e_unit_serialized = [0, 0, 0, 0];
        assert_eq!(
            expected_e_unit,
            from_slice(&e_unit_serialized).expect("could not deserialize")
        );

        // newtype
        let expected_e_newtype = E::Newtype(42);
        let e_newtype_serialized = [1, 0, 0, 0, 42, 0, 0, 0];
        assert_eq!(
            expected_e_newtype,
            from_slice(&e_newtype_serialized).expect("could not deserialize")
        );

        // tuple
        let expected_e_tuple = E::Tuple(42, 43);
        let e_tuple_serialized = [2, 0, 0, 0, 42, 0, 0, 0, 43, 0, 0, 0];
        assert_eq!(
            expected_e_tuple,
            from_slice(&e_tuple_serialized).expect("could not deserialize")
        );

        // struct
        let expected_e_struct = E::Struct { a: 42 };
        let e_struct_serialized = [3, 0, 0, 0, 42, 0, 0, 0];
        assert_eq!(
            expected_e_struct,
            from_slice(&e_struct_serialized).expect("could not deserialize")
        );

        // nested
        let expected_e_nested = E::Nested(Nested {
            int: 42,
            seq: vec![1, 2, 3],
        });
        let e_nested_serialized = [4, 0, 0, 0, 42, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3];
        assert_eq!(
            expected_e_nested,
            from_slice(&e_nested_serialized).expect("could not deserialize")
        );
    }
}
