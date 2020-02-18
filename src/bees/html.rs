use crate::helpers::fix_buf::FixBuf;

pub struct HtmlEscape<C>
where
    C: Iterator<Item = char>,
{
    base: std::iter::Flatten<Inner<C>>,
}

impl<C> HtmlEscape<C>
where
    C: Iterator<Item = char>,
{
    pub fn new<I>(chars: I) -> Self
    where
        I: IntoIterator<IntoIter = C, Item = char>,
    {
        HtmlEscape {
            base: Inner(chars.into_iter()).flatten(),
        }
    }
}

impl<C> Iterator for HtmlEscape<C>
where
    C: Iterator<Item = char>,
{
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.base.next()
    }
}

struct Inner<C>(C);

impl<C> Iterator for Inner<C>
where
    C: Iterator<Item = char>,
{
    type Item = FixBuf<[char; 6]>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|c| match c {
            '&' => FixBuf::new("&amp;".chars()),
            '<' => FixBuf::new("&lt;".chars()),
            '>' => FixBuf::new("&gt;".chars()),
            '"' => FixBuf::new("&quot;".chars()),
            '\'' => FixBuf::new("&apos;".chars()),
            _ => FixBuf::new(std::iter::once(c)),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::HtmlEscape;

    #[test]
    fn once() {
        assert_eq!(encode("hello"), "hello");
        assert_eq!(encode("<b>hello</b>"), "&lt;b&gt;hello&lt;/b&gt;");
        assert_eq!(encode("do & don't"), "do &amp; don&apos;t");
    }

    #[test]
    fn twice() {
        assert_eq!(encode_twice("hello"), "hello");
        assert_eq!(
            encode_twice("<b>hello</b>"),
            "&amp;lt;b&amp;gt;hello&amp;lt;/b&amp;gt;"
        );
        assert_eq!(encode_twice("do & don't"), "do &amp;amp; don&amp;apos;t");
    }

    fn encode(s: &str) -> String {
        HtmlEscape::new(s.chars()).collect()
    }

    fn encode_twice(s: &str) -> String {
        HtmlEscape::new(HtmlEscape::new(s.chars())).collect()
    }
}
