mod impls_int;
mod impls_array;
mod impls_tuple;
mod impls_bytes;
mod impls_other;
mod impls_string;
mod impls_float;
mod impls_hashmap;
mod impls;

use crate::{ContainerAttrModifiers, FieldAttrModifiers};


pub trait ByteEncode {
    fn encode(&self, input: &mut Vec<u8>, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>)
    // where 
    //     Self: Sized
    ;
}


pub trait BorrowByteEncode {
    fn encode(&self, input: &mut Vec<u8>, cattr: Option<&ContainerAttrModifiers>, fattr: Option<&FieldAttrModifiers>)
    // where 
    //     Self: Sized
    ;
}
