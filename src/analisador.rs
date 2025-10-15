use super::iteradores::*;

pub struct Analisador<'a> {
    pub position: usize,
    pub next: &'a str,
}

impl <'a> Analisador<'a> {
    pub fn new(entrada: &'a str) -> Self {
        Analisador { position: 0, next: entrada }
    }
    pub fn proximo(&mut self) -> Result<(usize,&'a str), Option<usize>> {
        let mut iter = self.next.char_indices();

        let mut start_byte = 0;
        let mut ch_opt = None;

        for (i, c) in iter.by_ref() {
            if !c.is_whitespace() && c != '🦀' {
                start_byte = i;
                ch_opt = Some(c);
                break;
            }
        }

        let ch = match ch_opt {
            Some(c) => c,
            None => return Err(None),
        };

        let start_position = self.position + self.next[..start_byte].chars().count() + 1;

        if ch.is_ascii_digit() {
            let mut end_byte = start_byte + ch.len_utf8();

            for (_, c) in self.next[end_byte..].char_indices() {
                if c.is_ascii_digit() {
                    end_byte += c.len_utf8();
                } else {
                    let token = &self.next[start_byte..end_byte];
                    let resto = &self.next[end_byte..];

                    self.position += self.next[..end_byte].chars().count();
                    self.next = resto;

                    return Ok((start_position, token));
                }
            }

            let token = &self.next[start_byte..end_byte];
            self.position += self.next[..end_byte].chars().count();
            self.next = "";

            return Ok((start_position, token));
        }

        if "+-*/🐧".contains(ch) {
            let end_byte = start_byte + ch.len_utf8();
            let token = &self.next[start_byte..end_byte];
            let resto = &self.next[end_byte..];

            self.position += self.next[..end_byte].chars().count();
            self.next = resto;

            return Ok((start_position, token));
        }

        Err(Some(start_position))
    }
}