
use jppe_derive::{ByteEncode, ByteDecode};


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
struct BranchOptionExample {
    pub flags: bool,
    #[jppe(if_expr="flags")]
    pub cmd: Option<u16>,
    #[jppe(branch_option="cmd")]
    pub body: BranchOptionExampleBody,
}


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
enum BranchOptionExampleBody {
    #[jppe(branch_value=0x0001)]
    Read {
        a: u8,
        b: u16,
    },
    #[jppe(branch_value=0x0002)]
    Write {
        a: u8,
        b: u16,
    },
    #[jppe(branch_default)]
    Unknown,
}


#[test]
fn test_modifier_branch_option() {
    let (input, value) = jppe::decode::<BranchOptionExample>(b"\x01\x00\x01\x02\x00\x03").unwrap();
    assert_eq!(value, BranchOptionExample { flags: true, cmd: Some(1), body: BranchOptionExampleBody::Read { a: 2, b: 3 } });
    assert_eq!(input.is_empty(), true);

    assert_eq!(jppe::encode(value), b"\x01\x00\x01\x02\x00\x03");
}
