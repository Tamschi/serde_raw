use serde::{ser, Serialize};
use std::{
	fmt::Error,
	io::{Seek, SeekFrom, Write},
};

pub fn to_raw(sink: &mut impl Write, value: impl Serialize) -> Result<(), Error> {
	value.serialize(Serializer(sink))
}

pub fn to_raw_at(
	sink: &mut (impl Write + Seek),
	value: impl Serialize,
	pos: SeekFrom,
) -> Result<(), Error> {
	value.serialize(Serializer(sink))
}

struct Serializer<'a, Sink: Write>(&'a mut Sink);
impl<'a, Sink: Write> Serializer<'a, Sink> {
	fn by_ref(&mut self) -> Serializer<Sink> {
		Serializer(self.0)
	}
}
impl<'a, Sink: Write> ser::Serializer for Serializer<'a, Sink> {
	type Ok = ();
	type Error = Error;
	type SerializeSeq = Unsupported;
	type SerializeTuple = Self;
	type SerializeTupleStruct = Self;
	type SerializeTupleVariant = Unsupported;
	type SerializeMap = Unsupported;
	type SerializeStruct = Self;
	type SerializeStructVariant = Unsupported;

	fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
		todo!()
	}
	fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
		todo!()
	}
	fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
		todo!()
	}
	fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
		todo!()
	}
	fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
		todo!()
	}
	fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
		todo!()
	}
	fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
		todo!()
	}
	fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
		todo!()
	}
	fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
		todo!()
	}
	fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
		todo!()
	}
	fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
		todo!()
	}
	fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
		todo!()
	}
	fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
		todo!()
	}
	fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
		todo!()
	}
	fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
		todo!()
	}
	fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
	where
		T: Serialize,
	{
		todo!()
	}
	fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
		todo!()
	}
	fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
		todo!()
	}
	fn serialize_unit_variant(
		self,
		name: &'static str,
		variant_index: u32,
		variant: &'static str,
	) -> Result<Self::Ok, Self::Error> {
		todo!()
	}
	fn serialize_newtype_struct<T: ?Sized>(
		self,
		name: &'static str,
		value: &T,
	) -> Result<Self::Ok, Self::Error>
	where
		T: Serialize,
	{
		todo!()
	}
	fn serialize_newtype_variant<T: ?Sized>(
		self,
		name: &'static str,
		variant_index: u32,
		variant: &'static str,
		value: &T,
	) -> Result<Self::Ok, Self::Error>
	where
		T: Serialize,
	{
		todo!()
	}
	fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
		todo!()
	}
	fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
		todo!()
	}
	fn serialize_tuple_struct(
		self,
		name: &'static str,
		len: usize,
	) -> Result<Self::SerializeTupleStruct, Self::Error> {
		todo!()
	}
	fn serialize_tuple_variant(
		self,
		name: &'static str,
		variant_index: u32,
		variant: &'static str,
		len: usize,
	) -> Result<Self::SerializeTupleVariant, Self::Error> {
		todo!()
	}
	fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
		todo!()
	}
	fn serialize_struct(
		self,
		name: &'static str,
		len: usize,
	) -> Result<Self::SerializeStruct, Self::Error> {
		todo!()
	}
	fn serialize_struct_variant(
		self,
		name: &'static str,
		variant_index: u32,
		variant: &'static str,
		len: usize,
	) -> Result<Self::SerializeStructVariant, Self::Error> {
		todo!()
	}
}

impl<'a, Sink: Write> ser::SerializeTuple for Serializer<'a, Sink> {
	type Ok = ();
	type Error = Error;
	fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
	where
		T: Serialize,
	{
		value.serialize(self.by_ref())
	}
	fn end(self) -> Result<Self::Ok, Self::Error> {
		Ok(())
	}
}
impl<'a, Sink: Write> ser::SerializeTupleStruct for Serializer<'a, Sink> {
	type Ok = ();
	type Error = Error;
	fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
	where
		T: Serialize,
	{
		value.serialize(self.by_ref())
	}
	fn end(self) -> Result<Self::Ok, Self::Error> {
		Ok(())
	}
}
impl<'a, Sink: Write> ser::SerializeStruct for Serializer<'a, Sink> {
	type Ok = ();
	type Error = Error;
	fn serialize_field<T: ?Sized>(
		&mut self,
		key: &'static str,
		value: &T,
	) -> Result<(), Self::Error>
	where
		T: Serialize,
	{
		value.serialize(self.by_ref())
	}
	fn end(self) -> Result<Self::Ok, Self::Error> {
		Ok(())
	}
	fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error> {
		unimplemented!("Skipping fields seems like a bad idea")
	}
}

enum Unsupported {}
impl ser::SerializeSeq for Unsupported {
	type Ok = ();
	type Error = Error;
	fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
	where
		T: Serialize,
	{
		unreachable!()
	}
	fn end(self) -> Result<Self::Ok, Self::Error> {
		unreachable!()
	}
}
impl ser::SerializeTupleVariant for Unsupported {
	type Ok = ();
	type Error = Error;
	fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
	where
		T: Serialize,
	{
		unreachable!()
	}
	fn end(self) -> Result<Self::Ok, Self::Error> {
		unreachable!()
	}
}
impl ser::SerializeMap for Unsupported {
	type Ok = ();
	type Error = Error;
	fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
	where
		T: Serialize,
	{
		todo!()
	}
	fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
	where
		T: Serialize,
	{
		todo!()
	}
	fn end(self) -> Result<Self::Ok, Self::Error> {
		todo!()
	}
}
impl ser::SerializeStructVariant for Unsupported {
	type Ok = ();
	type Error = Error;
	fn serialize_field<T: ?Sized>(
		&mut self,
		key: &'static str,
		value: &T,
	) -> Result<(), Self::Error>
	where
		T: Serialize,
	{
		unreachable!()
	}
	fn end(self) -> Result<Self::Ok, Self::Error> {
		unreachable!()
	}
	fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error> {
		unreachable!()
	}
}
