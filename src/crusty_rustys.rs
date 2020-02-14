use std::collections::VecDeque;
use std::error;
use std::fmt;

#[derive(Debug)]
struct HtmlDecoderError;

impl fmt::Display for HtmlDecoderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid escape sequence")
    }
}

impl error::Error for HtmlDecoderError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

#[test]
fn one_plus_one_is_two() {
    assert_eq!(1 + 1, 2);
}

fn html_encode(raw: &str) -> String {
    let mut s: String = String::new();
    for c in raw.chars() {
        match c {
            '<' => {
                s.push_str("&lt;");
            }
            '>' => {
                s.push_str("&gt;");
            }
            '"' => {
                s.push_str("&quo;");
            }
            '\'' => {
                s.push_str("&apos;");
            }
            '&' => {
                s.push_str("&amp;");
            }
            _ => {
                s.push(c);
            }
        }
    }
    s
}

fn html_decode(html: &str) -> Result<String, HtmlDecoderError> {
    let mut s: String = String::new();
    for c in html.chars() {
        match c {
            '<' => {
                s.push_str("&lt;");
            }
            '>' => {
                s.push_str("&gt;");
            }
            '"' => {
                s.push_str("&quo;");
            }
            '\'' => {
                s.push_str("&apos;");
            }
            '&' => {
                s.push_str("&amp;");
            }
            _ => {
                s.push(c);
            }
        }
    }
    Ok(s)
}

#[test]
fn test_encoder() {
    assert_eq!(
        html_encode("hello michael & I don't know how html works \' but I'll keep typing spec><ial characters"),
        "hello michael &amp; I don&apos;t know how html works &apos; but I&apos;ll keep typing spec&gt;&lt;ial characters"
    );
}

// Iterator method

fn html_encode_is_special_char(c: char) -> Option<&'static str> {
    match c {
        '<' => Some("lt;"),
        '>' => Some("gt;"),
        '"' => Some("quo;"),
        '\'' => Some("apos;"),
        '&' => Some("amp;"),
        _ => None,
    }
}

struct HtmlEncoder<C> {
    char_iterator: C,
    escape_chars: VecDeque<char>, //state we need to do the iterator
                                  // C is the type
}
struct HtmlDecoder<C> {
    char_iterator: C,
    skipped_chars: VecDeque<char>,
}

impl<C: Iterator<Item = char>> HtmlEncoder<C> {
    // this is a method that applies to any HtmlEncoder<C> where C is a char iterator
    pub fn new<I>(chars: I) -> Self
    where
        I: IntoIterator<Item = char, IntoIter = C>,
    {
        HtmlEncoder {
            char_iterator: chars.into_iter(),
            escape_chars: VecDeque::new(),
        }
    }
}

impl<C: Iterator<Item = char>> HtmlDecoder<C> {
    pub fn new<I>(chars: I) -> Self
    where
        I: IntoIterator<Item = char, IntoIter = C>,
    {
        HtmlDecoder {
            char_iterator: chars.into_iter(),
            skipped_chars: VecDeque::new(),
        }
    }
}

impl<C> Iterator for HtmlEncoder<C>
where
    C: Iterator<Item = char>,
{
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(c) = self.escape_chars.pop_front() {
            return Some(c);
        } else {
            let next_char = self.char_iterator.next()?;
            if let Some(s) = html_encode_is_special_char(next_char) {
                self.escape_chars.extend(s.chars());
                return Some('&');
            }
            return Some(next_char);
        }
    }
}

impl<C> Iterator for HtmlDecoder<C>
where
    C: Iterator<Item = char>,
{
    type Item = Result<char, HtmlDecoderError>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(c) = self.skipped_chars.pop_front() {
            return Some(Ok(c));
        } else {
            let next_char = self.char_iterator.next()?;
            if next_char == '&' {}
            return Some(Ok(next_char));
        }
    }
}

#[test]
fn test_iter_encoder() {
    let enc = HtmlEncoder::new(
        "hello michael & I don't know how html works \' but I'll keep typing spec><ial characters"
            .chars(),
    );
    assert_eq!(enc.collect::<String>(), String::from("hello michael &amp; I don&apos;t know how html works &apos; but I&apos;ll keep typing spec&gt;&lt;ial characters"));
}
/*
#[test]
fn iter_decodes_correct_str() {
    let dec = HtmlDecoder::new("hello michael &amp; I don&apos;t know how html works &apos; but I&apos;ll keep typing spec&gt;&lt;ial characters".chars());
    let mut res = String::new();
    if let Ok(s) = dec.collect(){
        res = s;
    }
    assert_eq!(res, String::from("hello michael & I don't know how html works \' but I'll keep typing spec><ial characters"));
}
*/
