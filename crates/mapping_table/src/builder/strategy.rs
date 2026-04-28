/// Strategy used to infer mapping entries.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum MappingStrategy {
    /// Use txt file name as rename target
    /// e.g. "ultimate sit.txt" → "ultimate sit"
    TxtStem,

    /// Use txt file name with digits stripped
    /// e.g. "foo123.txt" → "foo"
    TxtStemStripped,

    /// Parse directory names like `123 - name`.
    DirPattern,
}
