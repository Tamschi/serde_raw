use serde::{de, Deserialize};
use std::{
	io::{Read, Seek, SeekFrom},
	marker::PhantomData,
	slice,
};

pub fn from_raw<'de, T: Deserialize<'de>>(
	source: &'de mut impl Read,
) -> Result<T, de::value::Error> {
	from_raw_seed(source, PhantomData)
}

pub fn from_raw_at<'de, T: Deserialize<'de>>(
	source: &'de mut (impl Read + Seek),
	pos: SeekFrom,
) -> Result<T, de::value::Error> {
	source.seek(pos).map_err(de::Error::custom)?;
	from_raw(source)
}

pub fn from_raw_seed<'de, Seed: de::DeserializeSeed<'de>>(
	source: &'de mut impl Read,
	seed: Seed,
) -> Result<Seed::Value, de::value::Error> {
	seed.deserialize(Deserializer(source))
}

pub fn from_raw_seed_at<'de, Seed: de::DeserializeSeed<'de>>(
	source: &'de mut (impl Read + Seek),
	seed: Seed,
	pos: SeekFrom,
) -> Result<Seed::Value, de::value::Error> {
	source.seek(pos).map_err(de::Error::custom)?;
	from_raw_seed(source, seed)
}

fn needs_clarification<T, E: de::Error>(exp: &dyn de::Expected) -> Result<T, E> {
	Err(de::Error::custom(format_args!(
		"Expected {}, but this type needs clarification",
		exp,
	)))
}

struct Deserializer<'a, Source: Read>(&'a mut Source);
impl<'a, 'de, Source: Read> de::Deserializer<'de> for Deserializer<'a, Source> {
	type Error = de::value::Error;
	fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		Err(de::Error::custom("This format is not self-describing"))
	}
	fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		needs_clarification(&visitor)
	}
	fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		needs_clarification(&visitor)
	}
	fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		needs_clarification(&visitor)
	}
	fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		needs_clarification(&visitor)
	}
	fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		needs_clarification(&visitor)
	}
	fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		needs_clarification(&visitor)
	}
	fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		let mut buf = 0;
		self.0
			.read_exact(&mut slice::from_mut(&mut buf))
			.map_err(de::Error::custom)?;
		visitor.visit_u8(buf)
	}
	fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		needs_clarification(&visitor)
	}
	fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		needs_clarification(&visitor)
	}
	fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		needs_clarification(&visitor)
	}
	fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		needs_clarification(&visitor)
	}
	fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		needs_clarification(&visitor)
	}
	fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		needs_clarification(&visitor)
	}
	fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		needs_clarification(&visitor)
	}
	fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		needs_clarification(&visitor)
	}
	fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		needs_clarification(&visitor)
	}
	fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		needs_clarification(&visitor)
	}
	fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		needs_clarification(&visitor)
	}
	fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		needs_clarification(&visitor)
	}
	fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		needs_clarification(&visitor)
	}
	fn deserialize_unit_struct<V>(
		self,
		_name: &'static str,
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		needs_clarification(&visitor)
	}
	fn deserialize_newtype_struct<V>(
		self,
		_name: &'static str,
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		visitor.visit_newtype_struct(self)
	}
	fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		needs_clarification(&visitor)
	}
	fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		visitor.visit_seq(TupleAccess(len, self.0))
	}
	fn deserialize_tuple_struct<V>(
		self,
		_name: &'static str,
		len: usize,
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		visitor.visit_seq(TupleAccess(len, self.0))
	}
	fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		needs_clarification(&visitor)
	}
	fn deserialize_struct<V>(
		self,
		_name: &'static str,
		fields: &'static [&'static str],
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		visitor.visit_seq(TupleAccess(fields.len(), self.0))
	}
	fn deserialize_enum<V>(
		self,
		_name: &'static str,
		_variants: &'static [&'static str],
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		needs_clarification(&visitor)
	}
	fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		needs_clarification(&visitor)
	}
	fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		needs_clarification(&visitor)
	}
	fn is_human_readable(&self) -> bool {
		false
	}
}

struct TupleAccess<'a, Source: Read>(usize, &'a mut Source);
impl<'a, 'de, Source: Read> de::SeqAccess<'de> for TupleAccess<'a, Source> {
	type Error = de::value::Error;
	fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
	where
		T: de::DeserializeSeed<'de>,
	{
		if self.0 > 0 {
			Some(seed.deserialize(Deserializer(self.1))).transpose()
		} else {
			Ok(None)
		}
	}
	fn size_hint(&self) -> Option<usize> {
		Some(self.0)
	}
}
