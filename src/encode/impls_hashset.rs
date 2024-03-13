// use std::collections::HashSet;


// impl<T: crate::ByteEncode> crate::ByteEncode for HashSet<T> {
//     fn encode(&self, input: &mut Vec<u8>, cattr: Option<&crate::ContainerAttrModifiers>, fattr: Option<&crate::FieldAttrModifiers>) {
//         for value in self {
//             T::encode(value, input, cattr, fattr);
//         }
//     }
// }


// impl<T: crate::BorrowByteEncode> crate::BorrowByteEncode for HashSet<T> {
//     fn encode(&self, input: &mut Vec<u8>, cattr: Option<&crate::ContainerAttrModifiers>, fattr: Option<&crate::FieldAttrModifiers>) {
//         for value in self {
//             T::encode(value, input, cattr, fattr);
//         }
//     }
// }


// #[cfg(test)]
// mod tests {
//     use crate::encode::ByteEncode;
//     use super::*;

//     #[test]
//     fn test_encode_hashset() {
//         let hashset = HashSet::<u16>::from([1, 2]);
//         let mut buf = vec![];
//         hashset.encode(&mut buf, None, None);
//         // hashset is out of order.
//         assert!(buf == b"\x00\x01\x00\x02" || buf == b"\x00\x02\x00\x01" );

//         let hashset = HashSet::from(["1".as_bytes(), b"2"]);
//         let mut buf = vec![];
//         hashset.encode(&mut buf, None, None);
//         // hashset is out of order.
//         assert!(buf == b"\x31\x32" || buf == b"\x32\x31" );
//     }
// }
