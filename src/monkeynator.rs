use std::collections::{HashMap, VecDeque};

pub struct HtmlEncoder<C> {
    iter: C,
    buffer: VecDeque<char>,
    spec_char_map: HashMap<char, String>,
}

pub struct HtmlDecoder<C> {
    iter: C,
    spec_seq_map: HashMap<String, char>,
    buffer: VecDeque<char>,
    _buf_len: usize,
    _max_seq_len: u16,
}

impl<C: Iterator<Item = char>> HtmlDecoder<C> {
    pub fn new<I>(chars: I) -> Self
    where
        I: IntoIterator<Item = char, IntoIter = C>,
    {
        let mut spec_seq_map = HashMap::new();
        spec_seq_map.insert("&gt;".to_string(), '>');
        spec_seq_map.insert("&lt;".to_string(), '<');
        spec_seq_map.insert("&apos;".to_string(), '\'');
        spec_seq_map.insert("&quot;".to_string(), '\"');
        spec_seq_map.insert("&amp;".to_string(), '&');

        return HtmlDecoder {
            iter: chars.into_iter(),
            _max_seq_len: 4,
            buffer: VecDeque::new(),
            _buf_len: 0,
            spec_seq_map,
        };
    }
}

impl<C: Iterator<Item = char>> HtmlEncoder<C> {
    pub fn new<I>(chars: I) -> Self
    where
        I: IntoIterator<Item = char, IntoIter = C>,
    {
        let mut spec_char_map = HashMap::new();
        spec_char_map.insert('>', "gt".to_string());
        spec_char_map.insert('<', "lt".to_string());
        spec_char_map.insert('\'', "apos".to_string());
        spec_char_map.insert('\"', "quot".to_string());
        spec_char_map.insert('&', "amp".to_string());

        return HtmlEncoder {
            iter: chars.into_iter(),
            buffer: VecDeque::new(),
            spec_char_map,
        };
    }
}

impl<C> Iterator for HtmlDecoder<C>
where
    C: Iterator<Item = char>,
{
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(dec_char) = self.buffer.pop_front() {
            return Some(dec_char);
        }
        // once we find a '&' we need to check for special sequence
        // if the sequence that follows is not special
        return match self.iter.next() {
            Some(next_char) => {
                // check for special sequence
                if next_char == '&' {
                    while let Some(c) = self.iter.next() {
                        self.buffer.push_back(c);
                        if c == ';' {
                            let mut buffer_contents: String = self.buffer.iter().collect();
                            buffer_contents.insert(0, '&');
                            if let Some(dec_char) = self.spec_seq_map.get(&buffer_contents) {
                                self.buffer.clear();
                                return Some(*dec_char);
                            }
                            break;
                        }
                        if c == '&' {
                            break;
                        }
                    }
                }
                Some(next_char)
            }
            None => None,
        };
    }
}

impl<C> Iterator for HtmlEncoder<C>
where
    C: Iterator<Item = char>,
{
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        // not finished emitting old special sequence
        if let Some(c) = self.buffer.pop_front() {
            return Some(c);
        }
        return match self.iter.next() {
            Some(next_char) => {
                if let Some(seq) = self.spec_char_map.get(&next_char) {
                    for c in seq.chars() {
                        self.buffer.push_back(c);
                    }
                    self.buffer.push_back(';');
                    Some('&')
                } else {
                    // not special
                    Some(next_char)
                }
            }
            None => None,
        };
    }
}

#[cfg(test)]
mod html_encode_tests {
    use crate::HtmlEncoder;

    #[test]
    fn test_basic_encoding() {
        test_encoding("john", "john");
    }

    #[test]
    fn test_ampersand_encoding() {
        test_encoding("baker&", "baker&amp;");
    }

    #[test]
    fn test_varied_encoding() {
        test_encoding(">me&", "&gt;me&amp;");
    }

    fn test_encoding(input: &str, expected: &str) {
        let mut encoder = HtmlEncoder::new(input.chars());
        let mut result: String = encoder.collect();
        assert_eq!(result.as_str(), expected);
    }
}

#[cfg(test)]
mod html_decode_tests {
    use crate::HtmlDecoder;

    #[test]
    fn test_basic_decoding() {
        test_decoding("john", "john");
    }

    #[test]
    fn test_ampersand_decoding() {
        test_decoding("baker&amp;", "baker&");
    }

    #[test]
    fn test_multiple_special_seqs() {
        test_decoding("&gt;me&amp;", ">me&");
    }

    #[test]
    fn test_invalid_outputs_itself() {
        test_decoding("&wrong;", "&wrong;");
    }

    #[test]
    fn test_wrong_first_correct_second() {
        test_decoding("&wrong;&amp;", "&wrong;&");
    }

    #[test]
    fn test_wrong_correct_second() {
        test_decoding("&w&amp;", "&w&");
    }

    fn test_decoding(input: &str, expected: &str) {
        let mut decoder = HtmlDecoder::new(input.chars());
        let mut result: String = decoder.collect();
        assert_eq!(result.as_str(), expected);
    }
}
