use bzw_highlight;

#[test]
fn lt() {
  assert_eq!(bzw_highlight::to_html("<"), "&lt;");
}

#[test]
fn gt() {
  assert_eq!(bzw_highlight::to_html(">"), "&gt;");
}

#[test]
fn symbol() {
  assert_eq!(bzw_highlight::to_html(r#"""#), r#"<span class="symbol">"</span>"#);
}

#[test]
fn comment() {
  assert_eq!(bzw_highlight::to_html("# test"), r#"<span class="comment"># test</span>"#);
}

#[test]
fn number() {
  assert_eq!(bzw_highlight::to_html("1"), r#"<span class="number">1</span>"#);
}

#[test]
fn header() {
  assert_eq!(bzw_highlight::to_html("world"), r#"<span class="header">world</span>"#);
}

#[test]
fn keyword() {
  assert_eq!(bzw_highlight::to_html("position"), r#"<span class="keyword">position</span>"#);
}
