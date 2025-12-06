use crate::iteradores::StrExt;

pub struct Analisador<'a> {
    pos: usize,     
    prox: &'a str,   
}

impl<'a> Analisador<'a> {
    pub fn novo(entrada: &'a str) -> Self {
        Analisador {
            pos: 1,          
            prox: entrada,
        }
    }
    
    pub fn próximo(&mut self) -> Result<(usize, &str), Option<usize>> {
        let mut iter = self.prox.meus_char_indices();
        let mut start_byte = 0;
        let mut _start_char = 0;
        let mut ch_opt = None;
        
        while let Some((byte_idx, char_idx, ch)) = iter.next() {
            if ch.is_whitespace() || ch == '🦀' {
                self.pos += 1;
            } else {
                start_byte = byte_idx;
                _start_char = char_idx;
                ch_opt = Some(ch);
                break;
            }
        }
        let ch = match ch_opt {
            Some(c) => c,
            None => {
                self.prox = "";
                return Err(None);
            }
        };
        
        let token_pos = self.pos;
        if ch.is_ascii_digit() {
            let mut end_byte = start_byte + ch.len_utf8();
            let mut char_count = 1; 
            
            while let Some((byte_idx, _, next_ch)) = iter.next() {
                if next_ch.is_ascii_digit() {
                    end_byte = byte_idx + next_ch.len_utf8();
                    char_count += 1;
                } else {
                    break;
                }
            }
            
            let token = &self.prox[start_byte..end_byte];
            self.pos += char_count;
            self.prox = &self.prox[end_byte..];
            return Ok((token_pos, token));
        }
        
        if "+-*/".contains(ch) || ch == '🐧' {
            let end_byte = start_byte + ch.len_utf8();
            let token = &self.prox[start_byte..end_byte];
            self.pos += 1;
            self.prox = &self.prox[end_byte..];
            return Ok((token_pos, token));
        }
        
        let end_byte = start_byte + ch.len_utf8();
        self.pos += 1;
        self.prox = &self.prox[end_byte..];
        Err(Some(token_pos))
    }
}