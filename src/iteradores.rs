pub struct Chars<'a> {
    pub slice: &'a str,
}

pub struct CharIndices<'a> {
    pub slice: &'a str,
    pub byte_index: usize,
    pub char_index: usize,
}

pub trait StrExt {
    fn meus_chars(&self) -> Chars<'_>;
    fn meus_char_indices(&self) -> CharIndices<'_>;
    fn char_count(&self) -> usize;
}

impl StrExt for str {
    fn meus_chars(&self) -> Chars<'_> {
        Chars { slice: self }
    }

    fn meus_char_indices(&self) -> CharIndices<'_> {
        CharIndices { 
            slice: self, 
            byte_index: 0,
            char_index: 0,
        }
    }
    
    fn char_count(&self) -> usize {
        self.chars().count()
    }
}

impl<'a> Iterator for Chars<'a> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        if self.slice.is_empty() {
            return None;
        }

        let ch = self.slice.chars().next().unwrap();
        let len = ch.len_utf8();
        self.slice = &self.slice[len..];
        Some(ch)
    }
}

impl<'a> Iterator for CharIndices<'a> {
    type Item = (usize, usize, char); // (byte_index, char_index, char)
    
    fn next(&mut self) -> Option<(usize, usize, char)> {
        if self.slice.is_empty() {
            return None;
        }
        
        let ch = self.slice.chars().next().unwrap();
        let ch_len = ch.len_utf8();
        let byte_idx = self.byte_index;
        let char_idx = self.char_index;
        
        self.slice = &self.slice[ch_len..];
        self.byte_index += ch_len;
        self.char_index += 1;
        
        Some((byte_idx, char_idx, ch))
    }
}