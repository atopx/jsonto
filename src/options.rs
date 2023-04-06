use crate::hints::Hint;

/// Options for the code generation
///
/// Construct with `Options::default()`, and change any settings you care about.
#[non_exhaustive]
#[derive(Debug, PartialEq, Clone)]
pub struct Options {
    pub output_mode: OutputMode,
    pub use_default_for_missing_fields: bool,
    pub deny_unknown_fields: bool,
    pub(crate) allow_option_vec: bool,
    pub type_visibility: String,
    pub field_visibility: Option<String>,
    pub derives: String,
    pub property_name_format: Option<StringTransform>,
    pub(crate) hints: Vec<(String, Hint)>,
    pub unwrap: String,
    pub import_style: ImportStyle,
    pub collect_additional: bool,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            output_mode: OutputMode::Rust,
            use_default_for_missing_fields: false,
            deny_unknown_fields: false,
            allow_option_vec: false,
            type_visibility: "pub".into(),
            field_visibility: Some("pub".into()),
            derives: "Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize".into(),
            property_name_format: None,
            hints: Vec::new(),
            unwrap: "".into(),
            import_style: ImportStyle::AddImports,
            collect_additional: false,
        }
    }
}

/// How imports/external types should be handled by code generation
#[non_exhaustive]
#[derive(Debug, PartialEq, Clone)]
pub enum ImportStyle {
    /// Add import/use statements for any external types used
    AddImports,
    /// Assume import/use statements already exist where the generated code will be inserted
    AssumeExisting,
    /// Use fully qualified paths for any external type used
    QualifiedPaths,
}

impl ImportStyle {
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "add_imports" => Some(ImportStyle::AddImports),
            "assume_existing" => Some(ImportStyle::AssumeExisting),
            "qualified_paths" => Some(ImportStyle::QualifiedPaths),
            _ => None,
        }
    }
}

#[non_exhaustive]
#[derive(Debug, PartialEq, Clone)]
pub enum OutputMode {
    Rust,
    Python,
    Schema,
    TypescriptIFace,
    TypescriptTypes,
}


const RUST_STR: &str = "rust";
const PYTHON_STR: &str = "python";
const SCHEMA_STR: &str = "schema";
const TS_IFACE_STR: &str = "typescript-iface";
const TS_TYPES_STR: &str = "typescript-types";

impl OutputMode {
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            RUST_STR => Some(OutputMode::Rust),
            PYTHON_STR => Some(OutputMode::Python),
            SCHEMA_STR => Some(OutputMode::Schema),
            TS_IFACE_STR => Some(OutputMode::TypescriptIFace),
            TS_TYPES_STR => Some(OutputMode::TypescriptTypes),
            _ => None,
        }
    }

    // pub fn
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, PartialEq, Clone)]
pub enum StringTransform {
    LowerCase,
    UpperCase,
    PascalCase,
    CamelCase,
    SnakeCase,
    ScreamingSnakeCase,
    KebabCase,
    ScreamingKebabCase,
}

impl StringTransform {
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "lowercase" => Some(StringTransform::LowerCase),
            "uppercase" | "UPPERCASE" => Some(StringTransform::UpperCase),
            "pascalcase" | "uppercamelcase" | "PascalCase" => Some(StringTransform::PascalCase),
            "camelcase" | "camelCase" => Some(StringTransform::CamelCase),
            "snakecase" | "snake_case" => Some(StringTransform::SnakeCase),
            "screamingsnakecase" | "SCREAMING_SNAKE_CASE" => {
                Some(StringTransform::ScreamingSnakeCase)
            }
            "kebabcase" | "kebab-case" => Some(StringTransform::KebabCase),
            "screamingkebabcase" | "SCREAMING-KEBAB-CASE" => {
                Some(StringTransform::ScreamingKebabCase)
            }
            _ => None,
        }
    }
}
