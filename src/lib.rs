use regex::Regex;

const HEADERS: [&str; 30] = [
  "world",
  "options",
  "waterLevel",
  "dynamicColor",
  "textureMatrix",
  "material",
  "transform",
  "physics",
  "arc",
  "base",
  "box",
  "cone",
  "define",
  "group",
  "link",
  "mesh",
  "meshbox",
  "meshpyr",
  "pyramid",
  "sphere",
  "teleporter",
  "tetra",
  "weapon",
  "zone",
  "face",
  "endface",
  "enddef",
  "drawInfo",
  "lod",
  "end",
];

const KEYWORDS: [&str; 11] = [
  "position",
  "size",
  "shift",
  "rotation",
  "color",
  "name",
  "flagHeight",
  "from",
  "to",
  "noWalls",
  "freeCtfSpawns",
];

fn highlight_span(class: &str) -> String {
  format!("<span class=\"{}\">$1</span>", class)
}

pub fn to_html(text: &str) -> String {
  let symbol = highlight_span("symbol");
  let symbol_str = symbol.as_str();

  let comment = highlight_span("comment");
  let comment_str = comment.as_str();

  let number = highlight_span("number");
  let number_str = number.as_str();

  let header = highlight_span("header");
  let header_str = header.as_str();

  let keyword = highlight_span("keyword");
  let keyword_str = keyword.as_str();

  let mut ret = String::from(text);
  ret = (&Regex::new("<").unwrap().replace_all(ret.as_str(), "&lt;")).to_string();
  ret = (&Regex::new(">").unwrap().replace_all(ret.as_str(), "&gt;")).to_string();
  ret = (&Regex::new("([-.*/\"=]+?)").unwrap().replace_all(ret.as_str(), symbol_str)).to_string();
  ret = (&Regex::new("(#.*?)($|\n)").unwrap().replace_all(ret.as_str(), format!("{}{}", comment_str, "$2").as_str())).to_string();
  ret = (&Regex::new("([0-9]+)").unwrap().replace_all(ret.as_str(), number_str)).to_string();
  ret = (&Regex::new(format!("({})", HEADERS.join("|")).as_str()).unwrap().replace_all(ret.as_str(), header_str)).to_string();
  ret = (&Regex::new(format!("({})", KEYWORDS.join("|")).as_str()).unwrap().replace_all(ret.as_str(), keyword_str)).to_string();

  ret
}
