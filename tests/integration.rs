use assumptions::{Assume, Assumption, assume, assume_eq, assume_matches, assume_ne, assumption};

#[test]
fn assumption_literal() {
    let result: Result<(), Assumption> = (|| {
        assumption!("response should contain a `data` field");
    })();
    assert_eq!(
        result.unwrap_err().message(),
        "response should contain a `data` field"
    );
}

#[test]
fn assumption_literal_trailing_comma() {
    let result: Result<(), Assumption> = (|| {
        assumption!("response should contain a `data` field",);
    })();
    assert_eq!(
        result.unwrap_err().message(),
        "response should contain a `data` field"
    );
}

#[test]
fn assumption_inline_format_arg() {
    let field = "data";
    let result: Result<(), Assumption> = (|| {
        assumption!("response should contain a `{field}` field");
    })();
    assert_eq!(
        result.unwrap_err().message(),
        "response should contain a `data` field"
    );
}

#[test]
fn assumption_format_args() {
    let result: Result<(), Assumption> = (|| {
        assumption!("response should contain a `{}` field", "data");
    })();
    assert_eq!(
        result.unwrap_err().message(),
        "response should contain a `data` field"
    );
}

#[test]
fn assumption_format_args_trailing_comma() {
    let result: Result<(), Assumption> = (|| {
        assumption!("response should contain a `{}` field", "data",);
    })();
    assert_eq!(
        result.unwrap_err().message(),
        "response should contain a `data` field"
    );
}

#[test]
fn assumption_arbitrary_error() {
    let result: Result<(), std::io::Error> = (|| {
        assumption!(std::io::Error::other("unexpected end of stream"));
    })();
    assert!(result.is_err());
}

#[test]
fn assumption_arbitrary_error_trailing_comma() {
    let result: Result<(), std::io::Error> = (|| {
        assumption!(std::io::Error::other("unexpected end of stream"),);
    })();
    assert!(result.is_err());
}

#[test]
fn assume_does_not_error_when_condition_holds() {
    let panels = std::hint::black_box([()]);
    let result: Result<(), Assumption> = (|| {
        assume!(!panels.is_empty(), "episode should have at least one panel");
        Ok(())
    })();
    assert!(result.is_ok());
}

#[test]
fn assume_literal() {
    let panels: Vec<()> = vec![];
    let result: Result<(), Assumption> = (|| {
        assume!(!panels.is_empty(), "episode should have at least one panel");
        Ok(())
    })();
    assert_eq!(
        result.unwrap_err().message(),
        "`!panels.is_empty()`: episode should have at least one panel"
    );
}

#[test]
fn assume_literal_trailing_comma() {
    let panels: Vec<()> = vec![];
    let result: Result<(), Assumption> = (|| {
        assume!(!panels.is_empty(), "episode should have at least one panel",);
        Ok(())
    })();
    assert_eq!(
        result.unwrap_err().message(),
        "`!panels.is_empty()`: episode should have at least one panel"
    );
}

#[test]
fn assume_inline_format_arg() {
    let panels: Vec<()> = vec![];
    let result: Result<(), Assumption> = (|| {
        assume!(
            !panels.is_empty(),
            "episode should have at least one panel, got: {panels:?}"
        );
        Ok(())
    })();
    assert_eq!(
        result.unwrap_err().message(),
        "`!panels.is_empty()`: episode should have at least one panel, got: []"
    );
}

#[test]
fn assume_format_args() {
    let panels: Vec<()> = vec![];
    let result: Result<(), Assumption> = (|| {
        assume!(
            !panels.is_empty(),
            "episode should have at least one panel, got: {}",
            panels.len()
        );
        Ok(())
    })();
    assert_eq!(
        result.unwrap_err().message(),
        "`!panels.is_empty()`: episode should have at least one panel, got: 0"
    );
}

#[test]
fn assume_format_args_trailing_comma() {
    let panels: Vec<()> = vec![];
    let result: Result<(), Assumption> = (|| {
        assume!(
            !panels.is_empty(),
            "episode should have at least one panel, got: {}",
            panels.len(),
        );
        Ok(())
    })();
    assert_eq!(
        result.unwrap_err().message(),
        "`!panels.is_empty()`: episode should have at least one panel, got: 0"
    );
}

#[test]
fn assume_arbitrary_error() {
    let panels: Vec<()> = vec![];
    let result: Result<(), std::io::Error> = (|| {
        assume!(
            !panels.is_empty(),
            std::io::Error::other("episode should have at least one panel")
        );
        Ok(())
    })();
    assert!(result.is_err());
}

#[test]
fn assume_arbitrary_error_trailing_comma() {
    let panels: Vec<()> = vec![];
    let result: Result<(), std::io::Error> = (|| {
        assume!(
            !panels.is_empty(),
            std::io::Error::other("episode should have at least one panel"),
        );
        Ok(())
    })();
    assert!(result.is_err());
}

#[test]
fn assume_matches_no_guard_succeeds_on_matching_pattern() {
    let result: Result<(), Assumption> = (|| {
        let value: Option<u32> = Some(1);
        assume_matches!(value, Some(_), "response id should be present");
        Ok(())
    })();
    assert!(result.is_ok());
}

#[test]
fn assume_matches_no_guard_literal_on_mismatch() {
    let result: Result<(), Assumption> = (|| {
        let value: Option<u32> = None;
        assume_matches!(value, Some(_), "response id should be present");
        Ok(())
    })();
    assert_eq!(
        result.unwrap_err().message(),
        "response id should be present, got: `None`"
    );
}

#[test]
fn assume_matches_no_guard_literal_trailing_comma() {
    let result: Result<(), Assumption> = (|| {
        let value: Option<u32> = None;
        assume_matches!(value, Some(_), "response id should be present",);
        Ok(())
    })();
    assert_eq!(
        result.unwrap_err().message(),
        "response id should be present, got: `None`"
    );
}

#[test]
fn assume_matches_no_guard_inline_format_arg_on_mismatch() {
    let field = "id";
    let result: Result<(), Assumption> = (|| {
        let value: Option<u32> = None;
        assume_matches!(value, Some(_), "response `{field}` should be present");
        Ok(())
    })();
    assert_eq!(
        result.unwrap_err().message(),
        "response `id` should be present, got: `None`"
    );
}

#[test]
fn assume_matches_no_guard_format_args_on_mismatch() {
    let result: Result<(), Assumption> = (|| {
        let value: Option<u32> = None;
        assume_matches!(value, Some(_), "response `{}` should be present", "id");
        Ok(())
    })();
    assert_eq!(
        result.unwrap_err().message(),
        "response `id` should be present, got: `None`"
    );
}

#[test]
fn assume_matches_no_guard_format_args_trailing_comma() {
    let result: Result<(), Assumption> = (|| {
        let value: Option<u32> = None;
        assume_matches!(value, Some(_), "response `{}` should be present", "id",);
        Ok(())
    })();
    assert_eq!(
        result.unwrap_err().message(),
        "response `id` should be present, got: `None`"
    );
}

#[test]
fn assume_matches_no_guard_arbitrary_error_on_mismatch() {
    let result: Result<(), std::io::Error> = (|| {
        let value: Option<u32> = None;
        assume_matches!(
            value,
            Some(_),
            std::io::Error::other("response id should be present")
        );
        Ok(())
    })();
    assert!(result.is_err());
}

#[test]
fn assume_matches_no_guard_arbitrary_error_trailing_comma() {
    let result: Result<(), std::io::Error> = (|| {
        let value: Option<u32> = None;
        assume_matches!(
            value,
            Some(_),
            std::io::Error::other("response id should be present"),
        );
        Ok(())
    })();
    assert!(result.is_err());
}

#[test]
fn assume_matches_with_guard_succeeds_when_pattern_and_guard_hold() {
    let result: Result<(), Assumption> = (|| {
        let published: Option<i32> = Some(2024);
        assume_matches!(
            published,
            Some(year) if year >= 2014,
            "episode publish year should be at least 2014"
        );
        Ok(())
    })();
    assert!(result.is_ok());
}

#[test]
fn assume_matches_with_guard_literal_when_pattern_matches_but_guard_fails() {
    let result: Result<(), Assumption> = (|| {
        let published: Option<i32> = Some(2010);
        assume_matches!(
            published,
            Some(year) if year >= 2014,
            "episode publish year should be at least 2014"
        );
        Ok(())
    })();
    assert_eq!(
        result.unwrap_err().message(),
        "episode publish year should be at least 2014, got: `Some(2010)`"
    );
}

#[test]
fn assume_matches_with_guard_literal_trailing_comma() {
    let result: Result<(), Assumption> = (|| {
        let published: Option<i32> = Some(2010);
        assume_matches!(
            published,
            Some(year) if year >= 2014,
            "episode publish year should be at least 2014",
        );
        Ok(())
    })();
    assert_eq!(
        result.unwrap_err().message(),
        "episode publish year should be at least 2014, got: `Some(2010)`"
    );
}

#[test]
fn assume_matches_with_guard_literal_when_pattern_itself_fails() {
    let result: Result<(), Assumption> = (|| {
        let published: Option<i32> = None;
        assume_matches!(
            published,
            Some(year) if year >= 2014,
            "episode publish year should be at least 2014"
        );
        Ok(())
    })();
    assert_eq!(
        result.unwrap_err().message(),
        "episode publish year should be at least 2014, got: `None`"
    );
}

#[test]
fn assume_matches_with_guard_format_args_on_mismatch() {
    let min = 2014;
    let result: Result<(), Assumption> = (|| {
        let published: Option<i32> = Some(2010);
        assume_matches!(
            published,
            Some(year) if year >= 2014,
            "episode publish year should be at least {}",
            min
        );
        Ok(())
    })();
    assert_eq!(
        result.unwrap_err().message(),
        "episode publish year should be at least 2014, got: `Some(2010)`"
    );
}

#[test]
fn assume_matches_with_guard_format_args_trailing_comma() {
    let min = 2014;
    let result: Result<(), Assumption> = (|| {
        let published: Option<i32> = Some(2010);
        assume_matches!(
            published,
            Some(year) if year >= 2014,
            "episode publish year should be at least {}",
            min,
        );
        Ok(())
    })();
    assert_eq!(
        result.unwrap_err().message(),
        "episode publish year should be at least 2014, got: `Some(2010)`"
    );
}

#[test]
fn assume_matches_with_guard_arbitrary_error_on_mismatch() {
    let result: Result<(), std::io::Error> = (|| {
        let published: Option<i32> = Some(2010);
        assume_matches!(
            published,
            Some(year) if year >= 2014,
            std::io::Error::other("episode publish year should be at least 2014")
        );
        Ok(())
    })();
    assert!(result.is_err());
}

#[test]
fn assume_matches_with_guard_arbitrary_error_trailing_comma() {
    let result: Result<(), std::io::Error> = (|| {
        let published: Option<i32> = Some(2010);
        assume_matches!(
            published,
            Some(year) if year >= 2014,
            std::io::Error::other("episode publish year should be at least 2014"),
        );
        Ok(())
    })();
    assert!(result.is_err());
}

#[test]
fn assume_eq_does_not_error_when_equal() {
    let result: Result<(), Assumption> = (|| {
        assume_eq!(
            1u32,
            1u32,
            "item count should match the header's declared count"
        );
        Ok(())
    })();
    assert!(result.is_ok());
}

#[test]
fn assume_eq_literal_on_mismatch() {
    let result: Result<(), Assumption> = (|| {
        assume_eq!(
            1u32,
            2u32,
            "item count should match the header's declared count"
        );
        Ok(())
    })();
    assert_eq!(
        result.unwrap_err().message(),
        "item count should match the header's declared count: left = 1, right = 2"
    );
}

#[test]
fn assume_eq_literal_trailing_comma() {
    let result: Result<(), Assumption> = (|| {
        assume_eq!(
            1u32,
            2u32,
            "item count should match the header's declared count",
        );
        Ok(())
    })();
    assert_eq!(
        result.unwrap_err().message(),
        "item count should match the header's declared count: left = 1, right = 2"
    );
}

#[test]
fn assume_eq_inline_format_arg_on_mismatch() {
    let expected = 2u32;
    let result: Result<(), Assumption> = (|| {
        assume_eq!(
            1u32,
            expected,
            "item count should match the header's declared count of {expected}"
        );
        Ok(())
    })();
    assert_eq!(
        result.unwrap_err().message(),
        "item count should match the header's declared count of 2: left = 1, right = 2"
    );
}

#[test]
fn assume_eq_format_args_on_mismatch() {
    let result: Result<(), Assumption> = (|| {
        assume_eq!(
            1u32,
            2u32,
            "item count should match the header's declared count of {}",
            2u32
        );
        Ok(())
    })();
    assert_eq!(
        result.unwrap_err().message(),
        "item count should match the header's declared count of 2: left = 1, right = 2"
    );
}

#[test]
fn assume_eq_format_args_trailing_comma() {
    let result: Result<(), Assumption> = (|| {
        assume_eq!(
            1u32,
            2u32,
            "item count should match the header's declared count of {}",
            2u32,
        );
        Ok(())
    })();
    assert_eq!(
        result.unwrap_err().message(),
        "item count should match the header's declared count of 2: left = 1, right = 2"
    );
}

#[test]
fn assume_eq_arbitrary_error_on_mismatch() {
    let result: Result<(), std::io::Error> = (|| {
        assume_eq!(
            1u32,
            2u32,
            std::io::Error::other("item count should match the header's declared count")
        );
        Ok(())
    })();
    assert!(result.is_err());
}

#[test]
fn assume_eq_arbitrary_error_trailing_comma() {
    let result: Result<(), std::io::Error> = (|| {
        assume_eq!(
            1u32,
            2u32,
            std::io::Error::other("item count should match the header's declared count"),
        );
        Ok(())
    })();
    assert!(result.is_err());
}

#[test]
fn assume_ne_does_not_error_when_not_equal() {
    let result: Result<(), Assumption> = (|| {
        assume_ne!(1u32, 2u32, "regenerated id should differ from the original");
        Ok(())
    })();
    assert!(result.is_ok());
}

#[test]
fn assume_ne_literal_on_match() {
    let result: Result<(), Assumption> = (|| {
        assume_ne!(1u32, 1u32, "regenerated id should differ from the original");
        Ok(())
    })();
    assert_eq!(
        result.unwrap_err().message(),
        "regenerated id should differ from the original: left = 1, right = 1"
    );
}

#[test]
fn assume_ne_literal_trailing_comma() {
    let result: Result<(), Assumption> = (|| {
        assume_ne!(1u32, 1u32, "regenerated id should differ from the original",);
        Ok(())
    })();
    assert_eq!(
        result.unwrap_err().message(),
        "regenerated id should differ from the original: left = 1, right = 1"
    );
}

#[test]
fn assume_ne_inline_format_arg_on_match() {
    let old_id = 1u32;
    let result: Result<(), Assumption> = (|| {
        assume_ne!(
            old_id,
            1u32,
            "regenerated id should differ from original `{old_id}`"
        );
        Ok(())
    })();
    assert_eq!(
        result.unwrap_err().message(),
        "regenerated id should differ from original `1`: left = 1, right = 1"
    );
}

#[test]
fn assume_ne_format_args_on_match() {
    let result: Result<(), Assumption> = (|| {
        assume_ne!(
            1u32,
            1u32,
            "regenerated id should differ from original `{}`",
            1u32
        );
        Ok(())
    })();
    assert_eq!(
        result.unwrap_err().message(),
        "regenerated id should differ from original `1`: left = 1, right = 1"
    );
}

#[test]
fn assume_ne_format_args_trailing_comma() {
    let result: Result<(), Assumption> = (|| {
        assume_ne!(
            1u32,
            1u32,
            "regenerated id should differ from original `{}`",
            1u32,
        );
        Ok(())
    })();
    assert_eq!(
        result.unwrap_err().message(),
        "regenerated id should differ from original `1`: left = 1, right = 1"
    );
}

#[test]
fn assume_ne_arbitrary_error_on_match() {
    let result: Result<(), std::io::Error> = (|| {
        assume_ne!(
            1u32,
            1u32,
            std::io::Error::other("regenerated id should differ from the original")
        );
        Ok(())
    })();
    assert!(result.is_err());
}

#[test]
fn assume_ne_arbitrary_error_trailing_comma() {
    let result: Result<(), std::io::Error> = (|| {
        assume_ne!(
            1u32,
            1u32,
            std::io::Error::other("regenerated id should differ from the original"),
        );
        Ok(())
    })();
    assert!(result.is_err());
}

#[test]
fn option_assumption_is_ok_when_some() {
    let val: Option<()> = Some(());
    let result: Result<(), Assumption> = val.assumption("value should be present");
    assert!(result.is_ok());
}

#[test]
fn option_assumption_is_err_when_none() {
    let val: Option<()> = None;
    let result: Result<(), Assumption> = val.assumption("config entry should be present");
    assert_eq!(
        result.unwrap_err().message(),
        "config entry should be present"
    );
}

#[test]
fn option_with_assumption_is_ok_when_some() {
    let val: Option<()> = Some(());
    let result: Result<(), Assumption> =
        val.with_assumption(|| "value should be present".to_string());
    assert!(result.is_ok());
}

#[test]
fn option_with_assumption_is_err_when_none() {
    let key = "version";
    let val: Option<()> = None;
    let result: Result<(), Assumption> =
        val.with_assumption(|| format!("config should contain a `{key}` entry"));
    assert_eq!(
        result.unwrap_err().message(),
        "config should contain a `version` entry"
    );
}

#[test]
fn option_with_assumption_closure_not_called_on_some() {
    let val: Option<()> = Some(());
    let mut called = false;
    let _ = val.with_assumption(|| {
        called = true;
        "should not be called".to_string()
    });
    assert!(!called);
}

#[test]
fn result_assumption_is_ok_when_ok() {
    let val: Result<(), &str> = Ok(());
    let result: Result<(), Assumption> = val.assumption("value should be present");
    assert!(result.is_ok());
}

#[test]
fn result_assumption_appends_original_error() {
    let val: Result<(), &str> = Err("unexpected end of input");
    let result: Result<(), Assumption> = val.assumption("config file should be valid JSON");
    assert_eq!(
        result.unwrap_err().message(),
        "config file should be valid JSON: unexpected end of input"
    );
}

#[test]
fn result_with_assumption_is_ok_when_ok() {
    let val: Result<(), &str> = Ok(());
    let result: Result<(), Assumption> =
        val.with_assumption(|| "value should be present".to_string());
    assert!(result.is_ok());
}

#[test]
fn result_with_assumption_appends_original_error() {
    let webtoon = "tower-of-god";
    let val: Result<(), &str> = Err("unexpected end of input");
    let result: Result<(), Assumption> =
        val.with_assumption(|| format!("page for `{webtoon}` should be valid HTML"));
    assert_eq!(
        result.unwrap_err().message(),
        "page for `tower-of-god` should be valid HTML: unexpected end of input"
    );
}

#[test]
fn result_with_assumption_closure_not_called_on_ok() {
    let val: Result<(), &str> = Ok(());
    let mut called = false;
    let _ = val.with_assumption(|| {
        called = true;
        "should not be called".to_string()
    });
    assert!(!called);
}

#[test]
fn assumption_display_includes_location_and_message() {
    let err = Assumption::new("episode should have at least one panel".to_string());
    let rendered = err.to_string();
    assert!(rendered.starts_with("internal assumption violated at "));
    assert!(rendered.ends_with("episode should have at least one panel"));
}

#[test]
fn assumption_message_accessor_returns_message_without_prefix() {
    let err = Assumption::new("episode should have at least one panel".to_string());
    assert_eq!(err.message(), "episode should have at least one panel");
}

#[test]
fn location_points_to_assumption_new_call_site() {
    let err = Assumption::new("episode should have at least one panel".to_string());
    assert_eq!(err.location().file(), "tests/integration.rs");
    assert_eq!(err.location().line(), 766);
}

#[test]
fn location_points_to_assumption_macro_call_site() {
    let result: Result<(), Assumption> = (|| {
        assumption!("episode should have at least one panel");
    })();
    let err = result.unwrap_err();
    assert_eq!(err.location().file(), "tests/integration.rs");
    assert_eq!(err.location().line(), 774);
}

#[test]
fn location_points_to_assume_macro_call_site() {
    let cond = std::hint::black_box(false);
    let result: Result<(), Assumption> = (|| {
        assume!(cond, "condition should hold");
        Ok(())
    })();
    let err = result.unwrap_err();
    assert_eq!(err.location().file(), "tests/integration.rs");
    assert_eq!(err.location().line(), 785);
}

#[test]
fn location_points_to_assume_matches_macro_call_site() {
    let cond = std::hint::black_box(Some(0));
    let result: Result<(), Assumption> = (|| {
        assume_matches!(cond, Some(1), "condition should hold");
        Ok(())
    })();
    let err = result.unwrap_err();
    assert_eq!(err.location().file(), "tests/integration.rs");
    assert_eq!(err.location().line(), 797);
}

#[test]
fn location_points_to_assume_eq_macro_call_site() {
    let cond = std::hint::black_box(false);
    let result: Result<(), Assumption> = (|| {
        assume_eq!(cond, true, "condition should hold");
        Ok(())
    })();
    let err = result.unwrap_err();
    assert_eq!(err.location().file(), "tests/integration.rs");
    assert_eq!(err.location().line(), 809);
}

#[test]
fn location_points_to_assume_ne_macro_call_site() {
    let cond = std::hint::black_box(false);
    let result: Result<(), Assumption> = (|| {
        assume_ne!(cond, false, "condition should hold");
        Ok(())
    })();
    let err = result.unwrap_err();
    assert_eq!(err.location().file(), "tests/integration.rs");
    assert_eq!(err.location().line(), 821);
}

#[test]
fn location_points_to_option_assumption_call_site() {
    let val: Option<()> = None;
    let result: Result<(), Assumption> = val.assumption("value should be present");
    let err = result.unwrap_err();
    assert_eq!(err.location().file(), "tests/integration.rs");
    assert_eq!(err.location().line(), 832);
}

#[test]
fn location_points_to_option_with_assumption_call_site() {
    let val: Option<()> = None;
    let result: Result<(), Assumption> =
        val.with_assumption(|| "value should be present".to_string());
    let err = result.unwrap_err();
    assert_eq!(err.location().file(), "tests/integration.rs");
    assert_eq!(err.location().line(), 842);
}

#[test]
fn location_points_to_result_assumption_call_site() {
    let val: Result<(), &str> = Err("original error");
    let result: Result<(), Assumption> = val.assumption("value should be present");
    let err = result.unwrap_err();
    assert_eq!(err.location().file(), "tests/integration.rs");
    assert_eq!(err.location().line(), 851);
}

#[test]
fn location_points_to_result_with_assumption_call_site() {
    let val: Result<(), &str> = Err("original error");
    let result: Result<(), Assumption> =
        val.with_assumption(|| "value should be present".to_string());
    let err = result.unwrap_err();
    assert_eq!(err.location().file(), "tests/integration.rs");
    assert_eq!(err.location().line(), 861);
}
