#[allow(dead_code)]
pub struct Chars<'a> {
    pub slice: &'a str,
}

pub struct CharIndices<'a> {
    pub slice: &'a str,
    pub index: usize,
}

#[allow(dead_code)]
pub trait StrExt {
    fn chars(&self) -> Chars<'_>;
    fn char_indices(&self) -> CharIndices<'_>;
}

impl StrExt for str {
    fn chars(&self) -> Chars<'_> {
        Chars { slice: self }
    }

    fn char_indices(&self) -> CharIndices<'_> {
        CharIndices { slice: self, index: 0 }
    }
}

impl<'a> Iterator for Chars<'a> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        if self.slice.len() == 0 {
            return None
        }

        let ch = self.slice.chars().next().unwrap();
        let len = ch.len_utf8();

        self.slice = &self.slice[len..];
        
        Some(ch)
    }
}

impl<'a> Iterator for CharIndices<'a> {
    type Item = (usize, char);
    
    fn next(&mut self) -> Option<(usize, char)> {
        if self.slice.len() == 0 {
            return None
        }
        
        let ch = self.slice.chars().next().unwrap();
        let ch_len = ch.len_utf8();
        let index = self.index;
        
        self.slice = &self.slice[ch_len..];
        self.index += ch_len;
        
        Some((index, ch))
    }
}

impl<'a> Clone for Chars<'a> {
    fn clone(&self) -> Chars<'a> {
        Chars { slice: self.slice }
    }
}
    
