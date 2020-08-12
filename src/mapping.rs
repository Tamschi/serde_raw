use serde::de;
use std::{borrow::Cow, fmt::Display};

pub trait Mapping<'de>: de::DeserializeSeed<'de> {}

pub fn fixed_len<'de>(
    len: usize,
) -> impl Mapping<'de> + de::DeserializeSeed<'de, Value = Cow<'de, [u8]>> {
    struct FixedLen(usize);
    impl<'de> Mapping<'de> for FixedLen {}
    impl<'de> de::DeserializeSeed<'de> for FixedLen {
        type Value = Cow<'de, [u8]>;
        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            struct Visitor(usize);
            impl<'de> serde::de::Visitor<'de> for Visitor {
                type Value = Cow<'de, [u8]>;
                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(formatter, "a {}-tuple", self.0)
                }

                fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    if v.len() == self.0 {
                        Ok(v.into())
                    } else {
                        Err(de::Error::invalid_length(v.len(), &self))
                    }
                }

                fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    if v.len() == self.0 {
                        Ok(v.into())
                    } else {
                        Err(de::Error::invalid_length(v.len(), &self))
                    }
                }

                fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    if v.len() == self.0 {
                        Ok(v.to_owned().into())
                    } else {
                        Err(de::Error::invalid_length(v.len(), &self))
                    }
                }

                fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                where
                    A: de::SeqAccess<'de>,
                {
                    let mut result = vec![];
                    for _ in 0..self.0 {
                        result.push(
                            seq.next_element()?
                                .ok_or_else(|| de::Error::invalid_length(result.len(), &self))?,
                        )
                    }
                    Ok(result.into())
                }
            }

            deserializer.deserialize_tuple(self.0, Visitor(self.0))
        }
    }

    FixedLen(len)
}

pub fn string<'de>(
    base: impl Mapping<'de> + de::DeserializeSeed<'de, Value = Cow<'de, [u8]>>,
    decode: impl for<'a> FnOnce(Cow<'a, [u8]>) -> Result<Cow<'a, str>, Box<dyn Display>>,
) -> impl Mapping<'de> + de::DeserializeSeed<'de, Value = Cow<'de, str>> {
    struct String<
        'de,
        Seed: de::DeserializeSeed<'de, Value = Cow<'de, [u8]>>,
        Decode: for<'a> FnOnce(Cow<'a, [u8]>) -> Result<Cow<'a, str>, Box<dyn Display>>,
    >(Seed, Decode, &'de ());
    impl<
            'de,
            Seed: de::DeserializeSeed<'de, Value = Cow<'de, [u8]>>,
            Decode: for<'a> FnOnce(Cow<'a, [u8]>) -> Result<Cow<'a, str>, Box<dyn Display>>,
        > Mapping<'de> for String<'de, Seed, Decode>
    {
    }
    impl<
            'de,
            Seed: de::DeserializeSeed<'de, Value = Cow<'de, [u8]>>,
            Decode: for<'a> FnOnce(Cow<'a, [u8]>) -> Result<Cow<'a, str>, Box<dyn Display>>,
        > de::DeserializeSeed<'de> for String<'de, Seed, Decode>
    {
        type Value = Cow<'de, str>;
        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let data = self.0.deserialize(deserializer)?;
            self.1(data).map_err(de::Error::custom)
        }
    }
    String(base, decode, &())
}
