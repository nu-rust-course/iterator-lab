use std::collections::{HashMap, VecDeque};
use std::io;
use std::io::{BufRead, Write};
use std::iter::FromIterator;

pub struct HtmlDecoderError {
    #[allow(dead_code)]
    msg: String,
}

impl HtmlDecoderError {
    pub fn new(msg: &str) -> Self {
        HtmlDecoderError {
            msg: String::from(msg),
        }
    }
}

pub fn html_encode_1(raw: &str) -> String {
    let encoder = HtmlEncoder::new(raw.chars());
    encoder.collect::<String>()
}

pub fn html_decode_1(html: &str) -> Result<String, HtmlDecoderError> {
    HtmlDecoder::new(html.chars()).collect()
}

pub fn html_encode_2(inp: impl BufRead, mut out: impl Write) -> Result<(), io::Error> {
    struct BufReadIter<C: BufRead> {
        item: C,
    }

    impl<C: BufRead> BufReadIter<C> {
        pub fn new(item: C) -> Self {
            BufReadIter { item }
        }
    }

    impl<C: BufRead> Iterator for BufReadIter<C> {
        type Item = char;

        fn next(&mut self) -> Option<Self::Item> {
            let mut b = [0u8];

            match self.item.read(&mut b) {
                Ok(n) => {
                    if n == 0 {
                        return None;
                    }
                    let c = char::from(b[0]);
                    Some(c)
                }
                Err(_) => {
                    // todo handle error
                    None
                }
            }
        }
    }
    let inp_iter = BufReadIter::new(inp);
    let encoder = HtmlEncoder::new(inp_iter);

    for v in encoder {
        let mut b = vec![0u8];
        v.encode_utf8(&mut b);
        if let Err(e) = out.write(&b) {
            return Err(e);
        }
    }
    if let Err(e) = out.flush() {
        return Err(e);
    }

    Ok(())
}

#[allow(unused_variables)]
pub fn html_decode_2(inp: impl BufRead, out: impl Write) -> Result<(), HtmlDecoderError> {
    todo!()
}

pub struct HtmlEncoder<C> {
    item: C,
    pool: VecDeque<char>,
}

pub struct HtmlDecoder<C> {
    item: C,
    decode_map: HashMap<&'static str, char>,
}

impl<C: Iterator<Item = char>> HtmlEncoder<C> {
    pub fn new<I>(chars: I) -> Self
    where
        I: IntoIterator<Item = char, IntoIter = C>,
    {
        HtmlEncoder {
            item: chars.into_iter(),
            pool: VecDeque::new(),
        }
    }
}

impl<C: Iterator<Item = char>> HtmlDecoder<C> {
    pub fn new<I>(chars: I) -> Self
    where
        I: IntoIterator<Item = char, IntoIter = C>,
    {
        fn get_decode_map() -> HashMap<&'static str, char> {
            let input = ['<', '>', '"', '\'', '&'];
            let output = ["&lt;", "&gt;", "&quot;", "&apos;", "&amp;"];
            output
                .iter()
                .map(|c| c.to_owned())
                .zip(input.iter().map(|s| s.to_owned()))
                .collect()
        }
        HtmlDecoder {
            item: chars.into_iter(),
            decode_map: get_decode_map(),
        }
    }
}

impl<C> Iterator for HtmlEncoder<C>
where
    C: Iterator<Item = char>,
{
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        use lazy_static::lazy_static;

        lazy_static! {
            static ref MAP: HashMap<char, &'static str> = [
                ('<', "&lt;"),
                ('>', "&gt;"),
                ('\'', "&apos;"),
                ('"', "&quot;"),
                ('&', "&amp;")
            ]
            .iter()
            .copied()
            .collect();
        }

        // pop from the pool first
        if let Some(v) = self.pool.pop_front() {
            return Some(v);
        }

        // if pool is empty, then read next()
        self.item.next().map(|c| {
            if let Some(v) = MAP.get(&c) {
                let mut iter = v.chars();
                let res = iter.next().unwrap();
                self.pool.extend(iter);
                res
            } else {
                c
            }
        })
    }
}

impl<C> Iterator for HtmlDecoder<C>
where
    C: Iterator<Item = char>,
{
    type Item = Result<char, HtmlDecoderError>;

    fn next(&mut self) -> Option<Self::Item> {
        fn generate_error() -> Option<Result<char, HtmlDecoderError>> {
            Some(Err(HtmlDecoderError::new("unrecognized char")))
        }

        let mut buf: String = String::new();

        while let Some(c) = self.item.next() {
            if !buf.is_empty() {
                // if sym.len() <= 6, it is valid
                // TODO calculate the max length
                if buf.len() > 6 {
                    return generate_error();
                }

                buf.push(c);

                if c == ';' {
                    match self.decode_map.get(buf.as_str()) {
                        Some(v) => return Some(Ok(*v)),
                        None => return generate_error(),
                    };
                }
            } else if c == '&' {
                buf.push(c);
            } else {
                return Some(Ok(c));
            }
        }

        if buf.is_empty() {
            None
        } else {
            generate_error()
        }
    }
}

#[cfg(test)]
mod iterator_test {
    use crate::{html_decode_1, html_encode_1, html_encode_2, HtmlDecoder};
    use std::borrow::ToOwned;
    use std::io::{BufReader, BufWriter};

    #[test]
    fn html_encode_1_test() {
        let input = "<>\"'&";

        let output = "&lt;&gt;&quot;&apos;&amp;";
        assert_eq!(output, html_encode_1(input));

        assert_eq!("12345", html_encode_1("12345"));
        assert_eq!("&amp;&lt;12345", html_encode_1("&<12345"));
    }

    #[test]
    fn html_decode_1_test() {
        let input = "<>\"'&";
        let output = "&lt;&gt;&quot;&apos;&amp;";

        assert_eq!(input, html_decode_1(output).unwrap_or_default());

        assert_eq!("12345", html_decode_1("12345").unwrap_or_default());
    }

    #[test]
    fn html_encode_2_test() {
        let input = "<>\"'&";
        let mut input_buf = BufReader::new(input.as_bytes());
        let output = "&lt;&gt;&quot;&apos;&amp;";
        let mut s = vec![];
        let mut output_buf = BufWriter::new(&mut s);
        html_encode_2(&mut input_buf, &mut output_buf).unwrap_or_else(|e| panic!(e));
        drop(output_buf);
        let res = String::from_utf8(s.to_owned()).unwrap();
        assert_eq!(res, output);
    }
}
