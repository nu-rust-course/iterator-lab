// use std::collections::VecDeque;
use thiserror::Error;

// use crate::E
// extern crate HtmlError;

// pub struct CharBuffer<'a> {
//      buffer: VecDeque<char>,
//     iter: std::str::Chars<'a>,
// }

// impl Iterator for CharBuffer {
//     // #[inline]
//     fn next(&mut self) -> Option<B> {
//         self.iter.next().map(&mut self.f)
//     }

//     // #[inline]
//     fn size_hint(&self) -> (usize, Option<usize>) {
//         self.iter.size_hint()
//     }
// }

// #[derive(Clone)]
// pub struct Map<I, F> {
//     iter: I,
//     f: F,
// }
// impl<I, F> Map<I, F> {
//     pub(super) fn new(iter: I, f: F) -> Map<I, F> {
//         Map { iter, f }
//     }
// }

// pub

#[derive(Debug, Error)]
pub enum HtmlError {
    #[error("no graph file given")]
    HtmlDecoderError,
    #[error("too many arguments given")]
    HtmlEncoderError,
}

pub struct HtmlEncoder<C> {
    buffer: std::str::Chars<'static>,
    iterator: C,
}

// pub struct HtmlDecoder<C> {
//     buffer: Option<std::str::Chars<'static>>,
//     iterator: C,
// }

pub struct HtmlDecoder<C> {
    buffer: String,
    iterator: C,
}

impl<'a, C: Iterator<Item = char>> HtmlEncoder<C> {
    pub fn new<I>(chars: I) -> Self
    where
        I: IntoIterator<Item = char, IntoIter = C>,
    {
        HtmlEncoder {
            buffer: "".chars(),
            iterator: chars.into_iter(),
        }
    }
}

impl<C: Iterator<Item = char>> HtmlDecoder<C> {
    pub fn new<I>(chars: I) -> Self
    where
        I: IntoIterator<Item = char, IntoIter = C>,
    {
        HtmlDecoder {
            buffer: String::new(),
            iterator: chars.into_iter(),
        }
    }
}

impl<C> Iterator for HtmlEncoder<C>
where
    C: Iterator<Item = char>,
{
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.buffer.next().or_else(|| {
            // there is a next char in the original iterator
            if let Some(c) = self.iterator.next() {
                if let Some(fresh) = parse_char(c) {
                    self.buffer = fresh;
                    // cannot be empty
                    self.buffer.next()
                } else {
                    Some(c)
                }
            } else {
                // the original iterator is empty and buffer is empty
                None
            }
        })
    }
}

impl<C> Iterator for HtmlDecoder<C>
where
    C: Iterator<Item = char>,
{
    type Item = Result<char, HtmlError>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(c) = self.iterator.next() {
            if c == '&' {
                // let mut buffer = String::new();
                let count = 0;

                while let Some(car) = self.iterator.next() {
                    if count > 5 {
                        break;
                    }
                    self.buffer.push(car);
                    if car == ';' {
                        break;
                    }
                }

                let result = Some(decode_html_amp(&self.buffer));
                self.buffer.clear();
                result
            } else {
                Some(Ok(c))
            }
        } else {
            None
        }
    }
}

fn decode_html_amp(s: &str) -> Result<char, HtmlError> {
    match s {
        "lt;" => Ok('<'),
        "gt;" => Ok('>'),
        "quot;" => Ok('\"'),
        "apos;" => Ok('\''),
        "amp;" => Ok('&'),
        _ => Err(HtmlError::HtmlDecoderError),
    }
}

// fn parse_char(c: char) -> Option<std::str::Chars<'static>> {
//     match c {
//         '<' => Some("&lt;".chars()),
//         '>' => Some("&gt;".chars()),
//         '"' => Some("&quot;".chars()),
//         '\'' => Some("&apos;".chars()),
//         '&' => Some("&amp;".chars()),
//         _ => None,
//     }
// }

#[test]
fn encode_empty_string() {
    let s = "";
    let res: String = HtmlEncoder::new(s.chars()).collect();
    assert_eq!(res, "");
}

#[test]
fn decode_empty_string() {
    let s = "";
    let res: Result<String, _> = HtmlDecoder::new(s.chars()).collect();
    assert_eq!(res.unwrap(), "");
}

#[test]
fn encode_hello_world() {
    let s = "<hello world>";
    let res: String = HtmlEncoder::new(s.chars()).collect();
    assert_eq!(res, "&lt;hello world&gt;")
}

#[test]
fn encode_nasty() {
    let s = "<l;ka>sldfkjsd<>Lkjsdf<><>slkjsdfkjsdf>\"\"\"\"\"\"&&&&&'";
    let res: String = HtmlEncoder::new(s.chars()).collect();
    assert_eq!(
        res,
        "&lt;l;ka&gt;sldfkjsd&lt;&gt;Lkjsdf&lt;&gt;&lt;&gt;slkjsdfkjsdf&gt;&quot;\
         &quot;&quot;&quot;&quot;&quot;&amp;&amp;&amp;&amp;&amp;&apos;"
    )
}

#[test]
fn decode_hello_world() {
    let s = "&lt;hello world&gt;";
    let res: Result<String, _> = HtmlDecoder::new(s.chars()).collect();
    assert_eq!(res.unwrap(), "<hello world>");
}

#[test]
fn decode_nasty() {
    let s = "&lt;l;ka&gt;sldfkjsd&lt;&gt;Lkjsdf&lt;&gt;&lt;&gt;slkjsdfkjsdf&gt;&quot;\
             &quot;&quot;&quot;&quot;&quot;&amp;&amp;&amp;&amp;&amp;&apos;";
    let res: Result<String, _> = HtmlDecoder::new(s.chars()).collect();
    assert_eq!(
        res.unwrap(),
        "<l;ka>sldfkjsd<>Lkjsdf<><>slkjsdfkjsdf>\"\"\"\"\"\"&&&&&'"
    );
}

#[test]
fn decode() {
    let s = "&rt";
    let res: Result<String, _> = HtmlDecoder::new(s.chars()).collect();
    assert!(res.is_err());
}

fn parse_char<'a>(c: char) -> Option<std::str::Chars<'a>> {
    match c {
        '<' => Some("&lt;".chars()),
        '>' => Some("&gt;".chars()),
        '"' => Some("&quot;".chars()),
        '\'' => Some("&apos;".chars()),
        '&' => Some("&amp;".chars()),
        _ => None,
    }
}

/*
fn html_encode(
    raw: impl IntoIterator<Item = char>,
) -> impl Iterator<Item = Result<char, crate::HtmlError>> {
    // raw.map(|x| )

    todo!();
}
fn html_decode(
    html: impl IntoIterator<Item = char>,
) -> impl Iterator<Item = Result<char, crate::HtmlError>> {
    todo!();
}*/

#[test]
fn one_plus_one_is_two() {
    assert_eq!(1 + 1, 2);
}
