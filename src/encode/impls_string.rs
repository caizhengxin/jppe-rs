use crate::{ByteEncode, BorrowByteEncode};
use crate::{int_to_vec, get_byteorder};


macro_rules! encode_string {
    ($value:expr, $input:expr, $cattr:expr, $fattr:expr) => {
        let mut byte_count_status = false;

        // key and split
        if let Some(fr) = $fattr {
            if let Some(byte_count) = fr.byte_count {
                $input.extend(int_to_vec($value.len(), byte_count, &get_byteorder($cattr, $fattr)));
                byte_count_status = true;
            }
            else if fr.linend_value.is_none() && fr.length.is_none() {
                $input.extend(int_to_vec($value.len(), 1, &crate::ByteOrder::Be));
                byte_count_status = true;
            }
            else {
                if let Some(key) = fr.key {
                    $input.extend(key);
                }

                if let Some(split) = fr.split  {
                    $input.extend(split);
                }    
            }
        }
        else {
            $input.extend(int_to_vec($value.len(), 1, &crate::ByteOrder::Be));
            byte_count_status = true;
        }

        // value
        $input.extend($value.as_bytes());

        if byte_count_status { return (); }

        if let Some(fr) = $fattr {
            if let Some(linend_value) = &fr.linend_value {
                $input.extend_from_slice(linend_value)
            }
            // else if fr.length.is_none() {
            //     $input.push(b'\n');
            // }
        }
        // else {
        //     $input.push(b'\n');
        // }    
    };
}


impl ByteEncode for String {
    fn encode(&self, input: &mut Vec<u8>, cattr: Option<&crate::ContainerAttrModifiers>, fattr: Option<&crate::FieldAttrModifiers>)
        where 
            Self: Sized
    {
        encode_string!(self, input, cattr, fattr);
    }
}


impl BorrowByteEncode for String {
    fn encode(&self, input: &mut Vec<u8>, cattr: Option<&crate::ContainerAttrModifiers>, fattr: Option<&crate::FieldAttrModifiers>)
        where 
            Self: Sized
    {
        encode_string!(self, input, cattr, fattr);
    }
}


impl ByteEncode for &str {
    fn encode(&self, input: &mut Vec<u8>, cattr: Option<&crate::ContainerAttrModifiers>, fattr: Option<&crate::FieldAttrModifiers>)
    where 
        Self: Sized
    {
        encode_string!(self, input, cattr, fattr);
    }
}


impl BorrowByteEncode for &str {
    fn encode(&self, input: &mut Vec<u8>, cattr: Option<&crate::ContainerAttrModifiers>, fattr: Option<&crate::FieldAttrModifiers>)
    where 
        Self: Sized
    {
        encode_string!(self, input, cattr, fattr);
    }
}
