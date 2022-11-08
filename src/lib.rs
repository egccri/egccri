//! # egccri
//! > egccri command lib.

#![allow(unknown_lints, bare_trait_objects, deprecated)]
#![cfg_attr(feature = "cargo-clippy", allow(renamed_and_removed_lints))]
#![cfg_attr(feature = "cargo-clippy", deny(clippy, clippy_pedantic))]
// Ignored clippy and clippy_pedantic lints
#![cfg_attr(
    feature = "cargo-clippy",
    allow(
        // clippy bug: https://github.com/rust-lang/rust-clippy/issues/5704
        unnested_or_patterns,
        // clippy bug: https://github.com/rust-lang/rust-clippy/issues/7768
        semicolon_if_nothing_returned,
        // not available in our oldest supported compiler
        checked_conversions,
        empty_enum,
        redundant_field_names,
        redundant_static_lifetimes,
        // integer and float ser/de requires these sorts of casts
        cast_possible_truncation,
        cast_possible_wrap,
        cast_sign_loss,
        // things are often more readable this way
        cast_lossless,
        module_name_repetitions,
        option_if_let_else,
        single_match_else,
        type_complexity,
        use_self,
        zero_prefixed_literal,
        // correctly used
        enum_glob_use,
        let_underscore_drop,
        map_err_ignore,
        result_unit_err,
        wildcard_imports,
        // not practical
        needless_pass_by_value,
        similar_names,
        too_many_lines,
        // preference
        doc_markdown,
        unseparated_literal_suffix,
        // false positive
        needless_doctest_main,
        // noisy
        missing_errors_doc,
        must_use_candidate,
    )
)]
// Rustc lints.
#![deny(missing_docs, unused_imports)]

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

static GLOBAL_DATA: Lazy<Mutex<HashMap<i32, String>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(13, "Spica".to_string());
    m.insert(74, "Hoyten".to_string());
    Mutex::new(m)
});

pub mod commands;
