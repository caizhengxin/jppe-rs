use core::slice::memchr;


/// Look for a substring in self
pub trait FindSubstring<T> {
/// Returns the byte position of the substring if it is found
fn find_substring(&self, substr: T) -> Option<usize>;
}
    
impl<'a, 'b> FindSubstring<&'b [u8]> for &'a [u8] {
    fn find_substring(&self, substr: &'b [u8]) -> Option<usize> {
        if substr.len() > self.len() {
            return None;
        }
    
        let (&substr_first, substr_rest) = match substr.split_first() {
            Some(split) => split,
            // an empty substring is found at position 0
            // This matches the behavior of str.find("").
            None => return Some(0),
        };
    
        if substr_rest.is_empty() {
            return memchr::memchr(substr_first, self);
        }
    
        let mut offset = 0;
        let haystack = &self[..self.len() - substr_rest.len()];
    
        while let Some(position) = memchr::memchr(substr_first, &haystack[offset..]) {
            offset += position;
            let next_offset = offset + 1;
            if &self[next_offset..][..substr_rest.len()] == substr_rest {
                return Some(offset);
            }
    
            offset = next_offset;
        }
    
        None
    }
}  
