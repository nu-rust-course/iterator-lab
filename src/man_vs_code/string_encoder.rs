//'static ALHPABET = &"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-._~:/?#[]@!$&'()*+,;=";
#[derive(PartialEq, Debug)]
enum HtmlDecoderError {
    Nothing,
}

fn str_html_encode(raw: &str) -> String {
    raw.replace(&"&", &"&amp;")
        .replace(&"<", &"&lt;")
        .replace(&">", &"&gt;")
        .replace(&"\"", &"&quot;")
        .replace(&"\'", &"&apos;")
}

fn str_html_decode(html: &str) -> Result<String, HtmlDecoderError> {
    Ok(html
        .replace(&"&lt;", &"<")
        .replace(&"&gt;", &">")
        .replace(&"&quot;", &"\"")
        .replace(&"&apos;", &"\'")
        .replace(&"&amp;", &"&"))
}

#[test]
fn test_str_encoder() {
    assert_eq!(
        str_html_encode(&"<test&>"),
        String::from("&lt;test&amp;&gt;")
    );
}

#[test]
fn test_str_decoder() {
    assert_eq!(
        str_html_decode(&"&apos;&amp;gt;ghab&apos;"),
        Ok(String::from("\'&gt;ghab\'"))
    );
}
