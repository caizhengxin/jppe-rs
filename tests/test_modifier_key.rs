
use jppe_derive::{ByteDecode, ByteEncode};


#[derive(Debug, PartialEq, Eq, ByteDecode, ByteEncode)]
pub struct KeyExample {
    #[jppe(key=b"Version: ", linend=b"\r\n")]
    pub version: String,
    #[jppe(key=b"Host: ", linend=b"\r\n")]
    pub host: String,
}


#[cfg(test)]
mod tests {
    use jppe::{ByteDecode, ByteEncode};

    use super::*;

    #[test]
    fn test_modifier_key() {
        let input = b"Cookie: sssss\r\nVersion: 1.0.0\r\nHeader: jkc\r\nHost: 192.168.1.1\r\nOther: jkc\r\n";
        let (input, value) = KeyExample::decode(input, None, None).unwrap();
        assert_eq!(value, KeyExample { version: "1.0.0".to_string(), host: "192.168.1.1".to_string() });
        assert_eq!(input, b"Other: jkc\r\n");

        // encode
        let mut buf = Vec::new();
        value.encode(&mut buf, None, None);
        assert_eq!(buf, b"Version: 1.0.0\r\nHost: 192.168.1.1\r\n");
    }
}