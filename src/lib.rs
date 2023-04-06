use thiserror::Error;

mod generation;
mod hints;
mod inference;
mod options;
mod shape;
mod word_case;

use crate::hints::Hints;
use crate::inference::shape_from_json;
pub use crate::options::{ImportStyle, Options, OutputMode, StringTransform};
pub use crate::shape::Shape;

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum JTError {
    #[error("An error occurred while parsing JSON")]
    JsonParsingError(#[from] inference::JsonInputErr),
}

/// The main code generation function for `json_typegen`
pub fn codegen(name: &str, input: &str, mut options: Options) -> Result<String, JTError> {
    let name = handle_pub_in_name(name, &mut options);
    let mut hints_vec = Vec::new();
    std::mem::swap(&mut options.hints, &mut hints_vec);
    let mut hints = Hints::new();
    for (pointer, hint) in hints_vec.iter() {
        hints.add(pointer, hint);
    }
    let shape = shape_from_json(input.as_bytes(), &options, &hints)?;
    codegen_from_shape(name, &shape, options)
}

/// Just code generation, no inference
pub fn codegen_from_shape(name: &str, shape: &Shape, options: Options) -> Result<String, JTError> {
    let mut generated_code = match options.output_mode {
        OutputMode::Rust => generation::rust::to(name, shape, options),
        OutputMode::Schema => generation::schema::to(name, shape, options),
        OutputMode::Python => generation::python::to(name, shape, options),
        OutputMode::TypescriptIFace => generation::typescript_iface::to(name, shape, options),
        OutputMode::TypescriptTypes => generation::typescript_types::to(name, shape, options),
    };

    // Ensure generated code ends with exactly one newline
    generated_code.truncate(generated_code.trim_end().len());
    generated_code.push('\n');

    Ok(generated_code)
}

/// Parse "names" like `pub(crate) Foo` into a name and a visibility option
fn handle_pub_in_name<'a>(name: &'a str, options: &mut Options) -> &'a str {
    if let Some(suffix) = name.strip_prefix("pub ") {
        options.type_visibility = "pub".to_string();
        return suffix;
    }
    if name.starts_with("pub(") {
        // MSRV: after 1.52 use split_once
        let split = name.splitn(2, ") ").collect::<Vec<_>>();
        if split.len() == 2 {
            options.type_visibility = format!("{})", split[0]);
            return split[1];
        }
    }
    name
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_pub_in_name() {
        let mut options = Options::default();
        let name = handle_pub_in_name("Foo", &mut options);
        assert_eq!(name, "Foo");
        assert_eq!(options.type_visibility, Options::default().type_visibility);
        let name = handle_pub_in_name("pub Foo", &mut options);
        assert_eq!(name, "Foo");
        assert_eq!(options.type_visibility, "pub".to_string());
        let name = handle_pub_in_name("pub(crate) Foo Bar", &mut options);
        assert_eq!(name, "Foo Bar");
        assert_eq!(options.type_visibility, "pub(crate)".to_string());
        let name = handle_pub_in_name("pub(some::path) Foo", &mut options);
        assert_eq!(name, "Foo");
        assert_eq!(options.type_visibility, "pub(some::path)".to_string());
    }

    #[test]
    fn test_codegen() {
        let input = r#"{"a": 1, "b": "2", "c": true}"#;
        let output = "use serde_derive::Deserialize;\nuse serde_derive::Serialize;\n\n#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]\npub struct Hello {\n    pub a: i64,\n    pub b: String,\n    pub c: bool,\n}\n";

        let mut options = Options::default();
        options.output_mode = OutputMode::Rust;
        let result = codegen("Hello", input, options).unwrap();
        assert_eq!(result.as_str(), output);
    }
}
