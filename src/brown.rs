#![cfg(feature = "hashbrown")]

/// Macro for creating a [map](std::collections::HashMap).
///
/// Equivalent to the [vec!] macro for [vectors](std::vec::Vec).
/// Set this [crate's](crate) documentation for more examples on how
/// to use this macro.
///
/// **Example:**
///
/// ```rust
/// use map_macro::brown_map;
///
/// let goodbye = brown_map! {
///     "en" => "Goodbye",
///     "de" => "Auf Wiedersehen",
///     "fr" => "Au revoir",
///     "es" => "Adios",
/// };
/// ```
///
#[macro_export]
macro_rules! brown_map {
    {$($k: expr => $v: expr),* $(,)?} => {
        hashbrown::HashMap::from([$(($k, $v),)*])
    };
}
