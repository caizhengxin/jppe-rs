use crate::std::*;
use crate::{ByteEncode, BorrowByteEncode};


impl<T, const N: usize> ByteEncode for [T; N]
where
    T: ByteEncode,
{
    fn encode(&self, input: &mut Vec<u8>, cattr: Option<&crate::ContainerAttrModifiers>, fattr: Option<&crate::FieldAttrModifiers>)
        where 
            Self: Sized
    {
        for v in self {
            v.encode(input, cattr, fattr)
        }
    }
}


impl<T, const N: usize> BorrowByteEncode for [T; N]
where
    T: BorrowByteEncode,
{
    fn encode(&self, input: &mut Vec<u8>, cattr: Option<&crate::ContainerAttrModifiers>, fattr: Option<&crate::FieldAttrModifiers>)
        where 
            Self: Sized
    {
        for v in self {
            v.encode(input, cattr, fattr)
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::ByteEncode;

    #[test]
    fn test_encode_array() {
        let mut buf = vec![];
        let value = [0x01_u8, 0x02];
        value.encode(&mut buf, None, None);
        assert_eq!(buf, [0x01, 0x02]);

        let mut buf = vec![];
        let value = [0x01_u16, 0x02];
        value.encode(&mut buf, None, None);
        assert_eq!(buf, [0x00, 0x01, 0x00, 0x02]);
    }
}