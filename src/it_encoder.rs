use std::borrow::Borrow;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, PartialEq)]
pub enum HtmlDecoderError {
    Mismatch(String),
}

lazy_static! {
    static ref ENCODE_MAP: HashMap<char, &'static str> = vec![
        ('<', "&lt;"),
        ('>', "&gt;"),
        ('\"', "&quot;"),
        ('\'', "&apos;"),
        ('&', "&amp;")
    ]
    .into_iter()
    .collect();
    static ref DECODE_MAP: HashMap<String, char> = vec![
        ("&lt;".to_string(), '<'),
        ("&gt;".to_string(), '>'),
        ("&quot;".to_string(), '\"'),
        ("&apos;".to_string(), '\''),
        ("&amp;".to_string(), '&')
    ]
    .into_iter()
    .collect();
}
/*
const decodeMap: HashMap<&str, &str> = vec![
    ()
].into_iter().collect();
*/

#[test]
fn one_plus_one_is_two() {
    assert_eq!(1 + 1, 2);
}

pub struct HtmlEncoder<C: Iterator> {
    raw: C,
    escape: VecDeque<C::Item>,
    //todo: std::str::Chars<'static>
}

pub struct HtmlDecoder<C> {
    raw: C,
}

impl<C: Iterator<Item = char>> HtmlEncoder<C> {
    pub fn new<I>(chars: I) -> Self
    where
        I: IntoIterator<Item = char, IntoIter = C>,
    {
        HtmlEncoder {
            raw: chars.into_iter(),
            escape: VecDeque::new(),
        }
    }
}

impl<'a, C: Iterator<Item = char>> HtmlDecoder<C> {
    pub fn new<I>(chars: I) -> Self
    where
        I: IntoIterator<Item = char, IntoIter = C>,
    {
        HtmlDecoder {
            raw: chars.into_iter(),
        }
    }

    fn verify_escape(&mut self) -> Result<char, HtmlDecoderError> {
        // if you call next on self.raw, you will get the char after the '&'
        let mut cur = String::from("&");
        loop {
            if let Some(c) = self.raw.next() {
                cur.push(c);
                if DECODE_MAP.keys().filter(|&s| s.starts_with(&cur)).count() == 0 {
                    break;
                }

                if let Some(c) = DECODE_MAP.get(&cur) {
                    return Ok(c.to_owned());
                }
            }
        }
        Err(HtmlDecoderError::Mismatch(cur))

        //        let cur = String::new();
        //        let mut pos = 0;
        //        cur.add("&");
        //        pos += 1;
        //        loop {
        //            match self.raw.next() {
        //
        //            }
        //        }
    }
}

impl<C> Iterator for HtmlEncoder<C>
where
    C: Iterator<Item = char>,
{
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(popChar) = self.escape.pop_front() {
            Some(popChar)
        } else {
            if let Some(c) = self.raw.next() {
                if let Some(val) = ENCODE_MAP.get(&c) {
                    self.escape = (*val).chars().into_iter().collect();
                    Some(self.escape.pop_front().unwrap())
                } else {
                    Some(c)
                }
            } else {
                //end of the raw
                None
            }
        }
    }
}

impl<'a, C> Iterator for HtmlDecoder<C>
where
    C: Iterator<Item = char>,
{
    type Item = Result<char, HtmlDecoderError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.raw.next().map(|c| {
            if c != '&' {
                Ok(c)
            } else {
                self.verify_escape()
            }
        })
    }
}

#[test]
fn test_html_encoder() {
    let htmlEcoder = HtmlEncoder::new("&<aaaa>".chars());
    let mut res = "&amp;&lt;aaaa&gt;".chars();
    for c in htmlEcoder.into_iter() {
        assert_eq!(c, res.next().unwrap());
    }
}

#[test]
fn test_html_decoder() {
    let htmlDecoder = HtmlDecoder::new("&amp;&lt;aaaa&gt;".chars());
    let mut res = "&<aaaa>".chars();
    for c in htmlDecoder {
        assert_eq!(c.unwrap(), res.next().unwrap())
    }
}

#[test]
fn test_html_decoder_err() {
    let mut htmlDecoder = HtmlDecoder::new("&am;&lt;aaaa&gt;".chars());
    let mut res = "<aaaa>".chars();
    assert_eq!(
        htmlDecoder.next().unwrap(),
        Err(HtmlDecoderError::Mismatch("&am;".to_string()))
    );
    for c in htmlDecoder {
        assert_eq!(c.unwrap(), res.next().unwrap())
    }
}
