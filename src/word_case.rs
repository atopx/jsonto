const ENDS_WITH_RULES: &[(&str, usize, &str)] = &[
    ("series", 0, ""),
    ("cookies", 1, ""),
    ("movies", 1, ""),
    ("ies", 3, "y"),
    ("les", 1, ""),
    ("pes", 1, ""),
    ("ss", 0, ""),
    ("es", 0, ""),
    ("is", 0, ""),
    ("as", 0, ""),
    ("us", 0, ""),
    ("os", 0, ""),
    ("news", 0, ""),
    ("s", 1, ""),
];

/// Singularize a word for use as a type name
///
/// Implementation notes:
/// - Prefer to be conservative; Missing singularizations are better than incorrect ones.
/// - It's OK if this is somewhat use-case specific. It's not exposed.
/// - No regexes, since we don't want the regex dependency in the WASM.
pub fn to_singular(s: &str) -> String {
    let lowercase = s.to_ascii_lowercase();
    for (suffix, to_strip, replacement) in ENDS_WITH_RULES {
        if lowercase.ends_with(suffix) {
            return s[0..(s.len() - to_strip)].to_string() + replacement;
        }
    }
    s.to_string()
}


pub fn camel_case(name: &str) -> String {
    let mut s = String::new();
    let mut last = ' ';
    for c in name.chars().skip_while(|c| !c.is_ascii_alphanumeric()) {
        if !c.is_ascii_alphanumeric() {
            last = c;
            continue;
        }
        if (last.is_ascii() && !last.is_ascii_alphanumeric() && c.is_ascii_alphanumeric())
            || (last.is_ascii_lowercase() && c.is_ascii_uppercase())
        {
            s.push(c.to_ascii_uppercase());
        } else if last.is_ascii_alphabetic() {
            s.push(c.to_ascii_lowercase());
        } else {
            s.push(c);
        }
        last = c;
    }
    s
}

pub fn snake_case(name: &str) -> String {
    sep_case(name, '_')
}

pub fn kebab_case(name: &str) -> String {
    sep_case(name, '-')
}

fn sep_case(name: &str, separator: char) -> String {
    let mut s = String::new();
    let mut last = 'A';
    for c in name.chars().skip_while(|c| !c.is_ascii_alphanumeric()) {
        if !c.is_ascii_alphanumeric() {
            last = c;
            continue;
        }
        if (last.is_ascii() && !last.is_ascii_alphanumeric() && c.is_ascii_alphanumeric())
            || (last.is_ascii_lowercase() && c.is_ascii_uppercase())
        {
            s.push(separator);
        }
        s.push(c.to_ascii_lowercase());
        last = c;
    }
    s
}

pub fn type_case(name: &str) -> String {
    let s = camel_case(name);
    uppercase_first_letter(&s)
}

pub fn lower_camel_case(name: &str) -> String {
    let s = camel_case(name);
    lowercase_first_letter(&s)
}

// from http://stackoverflow.com/questions/38406793/.../38406885
fn uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_ascii_uppercase().to_string() + c.as_str(),
    }
}

fn lowercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_ascii_lowercase().to_string() + c.as_str(),
    }
}

// based on hashmap! macro from maplit crate
macro_rules! string_hashmap {
    ($($key:expr => $value:expr,)+) => { string_hashmap!($($key => $value),+) };
    ($($key:expr => $value:expr),*) => {
        {
            let mut _map = ::linked_hash_map::LinkedHashMap::new();
            $(
                _map.insert($key.to_string(), $value);
            )*
            _map
        }
    };
}

pub(crate) use string_hashmap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camel_case() {
        assert_eq!("FooBar", &camel_case("FooBar"));
        assert_eq!("FooBar", &camel_case("fooBar"));
        assert_eq!("FooBar", &camel_case("foo bar"));
        assert_eq!("FooBar", &camel_case("foo_bar"));
        assert_eq!("FooBar", &camel_case("_foo_bar"));
        assert_eq!("FooBar", &camel_case("åfoo_bar"));
        assert_eq!("FooBar", &camel_case("foåo_bar"));
        assert_eq!("FooBar", &camel_case("FOO_BAR"));

        assert_eq!("Foo1bar", &camel_case("Foo1bar"));
        assert_eq!("Foo2bar", &camel_case("foo_2bar"));
        assert_eq!("Foo3Bar", &camel_case("Foo3Bar"));
        assert_eq!("Foo4Bar", &camel_case("foo4_bar"));
        assert_eq!("1920x1080", &camel_case("1920x1080"));
        assert_eq!("19201080", &camel_case("1920*1080"));
    }

    #[test]
    fn test_snake_case() {
        assert_eq!("foo_bar", &snake_case("FooBar"));
        assert_eq!("foo_bar", &snake_case("fooBar"));
        assert_eq!("foo_bar", &snake_case("foo bar"));
        assert_eq!("foo_bar", &snake_case("foo_bar"));
        assert_eq!("foo_bar", &snake_case("_foo_bar"));
        assert_eq!("foo_bar", &snake_case("åfoo_bar"));
        assert_eq!("foo_bar", &snake_case("foåo_bar"));
        assert_eq!("foo_bar", &snake_case("FOO_BAR"));

        assert_eq!("foo_5bar", &snake_case("foo_5bar"));
        assert_eq!("foo6_bar", &snake_case("foo6_bar"));
        assert_eq!("1920x1080", &snake_case("1920x1080"));
        assert_eq!("1920_1080", &snake_case("1920*1080"));
    }

    #[test]
    fn test_to_singular() {
        let mut incorrectly_singularized: Vec<(&'static str, &'static str, String)> = Vec::new();
        let mut not_singularized: Vec<(&'static str, &'static str)> = Vec::new();
        let mut assert_to_singular_matches = |input: &'static str, expected: &'static str| {
            let output = to_singular(input);
            if output != expected {
                if output == input {
                    not_singularized.push((input, expected))
                } else {
                    incorrectly_singularized.push((input, expected, output))
                }
            }
        };

        assert_to_singular_matches("cards", "card");
        assert_to_singular_matches("types", "type");
        assert_to_singular_matches("colors", "color");
        assert_to_singular_matches("rulings", "ruling");
        assert_to_singular_matches("foreignNames", "foreignName");
        assert_to_singular_matches("tags", "tag");
        assert_to_singular_matches("categoryKeys", "categoryKey");
        assert_to_singular_matches("attributes", "attribute");
        assert_to_singular_matches("values", "value");
        assert_to_singular_matches("images", "image");

        assert_to_singular_matches("guesses", "guess");

        assert_to_singular_matches("moves", "move");
        assert_to_singular_matches("lives", "life");
        assert_to_singular_matches("leaves", "leaf");

        assert_to_singular_matches("legalities", "legality");
        assert_to_singular_matches("abilities", "ability");
        assert_to_singular_matches("queries", "query");
        assert_to_singular_matches("cookies", "cookie");
        assert_to_singular_matches("movies", "movie");

        assert_to_singular_matches("matrices", "matrix");
        assert_to_singular_matches("vertices", "vertex");
        assert_to_singular_matches("indices", "index");
        assert_to_singular_matches("slices", "slice");

        assert_to_singular_matches("children", "child");

        assert_to_singular_matches("series", "series");
        assert_to_singular_matches("news", "news");
        assert_to_singular_matches("axis", "axis");

        if !not_singularized.is_empty() {
            println!(
                "Missed {} singularizations for to_singular() (input, expected):\n  {}\n\n",
                not_singularized.len(),
                not_singularized
                    .iter()
                    .map(|(input, expected)| format!("{}, {}", input, expected))
                    .collect::<Vec<_>>()
                    .join("\n  ")
            );
        }

        if !incorrectly_singularized.is_empty() {
            panic!(
                "Test failures for to_singular() (input, expected, output):\n  {}\n\n",
                incorrectly_singularized
                    .iter()
                    .map(|(input, expected, output)| format!("{}, {}, {}", input, expected, output))
                    .collect::<Vec<_>>()
                    .join("\n  ")
            );
        }
    }
}
