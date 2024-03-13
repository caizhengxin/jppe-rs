mod impls_int;
mod impls_string;
mod impls_bytes;
mod impls_array;
mod impls_tuple;
mod impls_float;
mod impls_hashmap;
mod impls_hashset;
mod impls_other;
mod impls_address;
mod impls;

use crate::{ContainerAttrModifiers, FieldAttrModifiers};
use crate::parser::JResult;


pub trait ByteDecode {
    fn decode<'da, 'db>(input: &'da [u8], cattr: Option<&'db ContainerAttrModifiers>, fattr: Option<&'db FieldAttrModifiers>) -> JResult<&'da [u8], Self>
    where 
        Self: Sized
    ;
}


pub trait BorrowByteDecode<'de> {
    fn decode<'da: 'de, 'db>(input: &'da [u8], cattr: Option<&'db ContainerAttrModifiers>, fattr: Option<&'db FieldAttrModifiers>) -> JResult<&'da [u8], Self>
    where 
        Self: Sized
    ;
}
