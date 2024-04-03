use crate::{ByteEncode, BorrowByteEncode};
use crate::{int_to_vec, get_byteorder};


// #[macro_export]
macro_rules! encode_bytes {
    ($value:expr, $input:expr, $cattr:expr, $fattr:expr) => {
        // key and split
        if let Some(fr) = $fattr {
            if let Some(byte_count) = fr.byte_count {
                $input.extend(int_to_vec($value.len(), byte_count, &get_byteorder($cattr, $fattr)));
            }
            else if fr.linend_value.is_none() && fr.length.is_none() {
                $input.extend(int_to_vec($value.len(), 1, &crate::ByteOrder::Be));
            }
            else {
                if let Some(key) = &fr.key { $input.extend(key); }
                if let Some(split) = fr.split {
                    $input.extend(split);
                }    
            }
        }
        // else {
        //     $input.extend(int_to_vec($value.len(), 1, &crate::ByteOrder::Be));
        // }

        $input.extend($value.to_vec());

        if let Some(fr) = $fattr && let Some(linend_value) = &fr.linend_value {
            $input.extend_from_slice(linend_value);
        }
    };
}


impl ByteEncode for &[u8] {
    fn encode(&self, input: &mut Vec<u8>, cattr: Option<&crate::ContainerAttrModifiers>, fattr: Option<&crate::FieldAttrModifiers>)
        where 
            Self: Sized
    {
        encode_bytes!(self , input, cattr, fattr);
    }
}


impl<'de> BorrowByteEncode for &'de [u8] {
    fn encode(&self, input: &mut Vec<u8>, cattr: Option<&crate::ContainerAttrModifiers>, fattr: Option<&crate::FieldAttrModifiers>)
        where 
            Self: Sized
    {
        encode_bytes!(self, input, cattr, fattr);
    }
}
