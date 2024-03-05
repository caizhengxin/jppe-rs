// ref: https://stackoverflow.com/questions/35901547/how-can-i-find-a-subsequence-in-a-u8-slice
#[inline(always)]
fn find_subsequence<T>(haystack: &[T], needle: &[T]) -> Option<usize>
where
    for<'a> &'a [T]: PartialEq,
{
    let needle_len = needle.len();
    haystack
        .windows(needle_len)
        .position(|window| window == needle)
}


fn main() {
    let input = [0x00, 0x01, 0x02, 0x00];

    println!("{:?}", &input[0..]);
    println!("{:?}", find_subsequence(&input, &[0x00]));
    println!("{:?}", find_subsequence(&input, &[0x00, 0x01]));
    println!("{:?}", find_subsequence(&input, &[0x01, 0x03]));
    println!("{:?}", find_subsequence(&input, &[0x01, 0x02]));
    println!("{:?}", find_subsequence(&input, &[0x01, 0x02, 0x00]));
}