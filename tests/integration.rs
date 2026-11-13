use assumptions::*;

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
    let err = Assumption::from("episode should have at least one panel");
    let rendered = err.to_string();
    assert!(rendered.starts_with("internal assumption violated at "));
    assert!(rendered.ends_with("episode should have at least one panel"));
}

#[test]
fn assumption_message_accessor_returns_message_without_prefix() {
    let err = Assumption::from("episode should have at least one panel");
    assert_eq!(err.message(), "episode should have at least one panel");
}

#[test]
fn location_points_to_assumption_new_call_site() {
    let line = line!() + 1;
    let err = Assumption::from("episode should have at least one panel");
    assert_eq!(err.location().file(), file!());
    assert_eq!(err.location().line(), line);
}

#[test]
fn location_points_to_assumption_macro_call_site() {
    let line = line!() + 2;
    let result: Result<(), Assumption> = (|| {
        assumption!("episode should have at least one panel");
    })();
    let err = result.unwrap_err();
    assert_eq!(err.location().file(), file!());
    assert_eq!(err.location().line(), line);
}

#[test]
fn location_points_to_assume_macro_call_site() {
    let cond = std::hint::black_box(false);
    let line = line!() + 2;
    let result: Result<(), Assumption> = (|| {
        assume!(cond, "condition should hold");
        Ok(())
    })();
    let err = result.unwrap_err();
    assert_eq!(err.location().file(), file!());
    assert_eq!(err.location().line(), line);
}

#[test]
fn location_points_to_assume_matches_macro_call_site() {
    let cond = std::hint::black_box(Some(0));
    let line = line!() + 2;
    let result: Result<(), Assumption> = (|| {
        assume_matches!(cond, Some(1), "condition should hold");
        Ok(())
    })();
    let err = result.unwrap_err();
    assert_eq!(err.location().file(), file!());
    assert_eq!(err.location().line(), line);
}

#[test]
fn location_points_to_assume_eq_macro_call_site() {
    let cond = std::hint::black_box(false);
    let line = line!() + 2;
    let result: Result<(), Assumption> = (|| {
        assume_eq!(cond, true, "condition should hold");
        Ok(())
    })();
    let err = result.unwrap_err();
    assert_eq!(err.location().file(), file!());
    assert_eq!(err.location().line(), line);
}

#[test]
fn location_points_to_assume_ne_macro_call_site() {
    let cond = std::hint::black_box(false);
    let line = line!() + 2;
    let result: Result<(), Assumption> = (|| {
        assume_ne!(cond, false, "condition should hold");
        Ok(())
    })();
    let err = result.unwrap_err();
    assert_eq!(err.location().file(), file!());
    assert_eq!(err.location().line(), line);
}

#[test]
fn location_points_to_assume_implies_macro_call_site() {
    let line = line!() + 2;
    let result: Result<(), Assumption> = (|| {
        assume_implies!(true => std::hint::black_box(false), "condition should hold");
        Ok(())
    })();
    let err = result.unwrap_err();
    assert_eq!(err.location().file(), file!());
    assert_eq!(err.location().line(), line);
}

#[test]
fn location_points_to_option_assumption_call_site() {
    let val: Option<()> = None;
    let line = line!() + 1;
    let result: Result<(), Assumption> = val.assumption("value should be present");
    let err = result.unwrap_err();
    assert_eq!(err.location().file(), file!());
    assert_eq!(err.location().line(), line);
}

#[test]
fn location_points_to_option_with_assumption_call_site() {
    let val: Option<()> = None;
    let line = line!() + 1;
    let result: Result<(), Assumption> = val.with_assumption(|| String::from("missing"));
    let err = result.unwrap_err();
    assert_eq!(err.location().file(), file!());
    assert_eq!(err.location().line(), line);
}

#[test]
fn location_points_to_result_assumption_call_site() {
    let val: Result<(), &str> = Err("original error");
    let line = line!() + 1;
    let result: Result<(), Assumption> = val.assumption("value should be present");
    let err = result.unwrap_err();
    assert_eq!(err.location().file(), file!());
    assert_eq!(err.location().line(), line);
}

#[test]
fn location_points_to_result_with_assumption_call_site() {
    let val: Result<(), &str> = Err("original error");
    let line = line!() + 1;
    let result: Result<(), Assumption> = val.with_assumption(|| String::from("missing"));
    let err = result.unwrap_err();
    assert_eq!(err.location().file(), file!());
    assert_eq!(err.location().line(), line);
}

#[test]
fn location_through_into_points_at_caller() {
    let line = line!() + 1;
    let err: Assumption = "episode should have at least one panel".into();
    assert_eq!(err.location().file(), file!());
    assert_eq!(err.location().line(), line);
}

#[test]
fn assume_implies_does_not_error_when_antecedent_is_false() {
    let result: Result<(), Assumption> = (|| {
        assume_implies!(false => false, "consequent should hold when antecedent is true");
        Ok(())
    })();
    assert!(result.is_ok());
}

#[test]
fn assume_implies_does_not_error_when_both_true() {
    let result: Result<(), Assumption> = (|| {
        assume_implies!(true => true, "consequent should hold when antecedent is true");
        Ok(())
    })();
    assert!(result.is_ok());
}

#[test]
fn assume_implies_errors_when_antecedent_true_and_consequent_false_literal() {
    let result: Result<(), Assumption> = (|| {
        assume_implies!(
            true => false,
            "consequent should hold when antecedent is true"
        );
        Ok(())
    })();
    assert_eq!(
        result.unwrap_err().message(),
        "`true => false`: consequent should hold when antecedent is true"
    );
}

#[test]
fn assume_implies_literal_trailing_comma() {
    let result: Result<(), Assumption> = (|| {
        assume_implies!(
            true => false,
            "consequent should hold when antecedent is true",
        );
        Ok(())
    })();
    assert_eq!(
        result.unwrap_err().message(),
        "`true => false`: consequent should hold when antecedent is true"
    );
}

#[test]
fn assume_implies_inline_format_arg() {
    let name = "published_at";
    let result: Result<(), Assumption> = (|| {
        assume_implies!(
            true => false,
            "published episode should always have a `{name}` date"
        );
        Ok(())
    })();
    assert_eq!(
        result.unwrap_err().message(),
        "`true => false`: published episode should always have a `published_at` date"
    );
}

#[test]
fn assume_implies_format_args() {
    let field = "published_at";
    let result: Result<(), Assumption> = (|| {
        assume_implies!(
            true => false,
            "published episode should always have a `{}` date",
            field
        );
        Ok(())
    })();
    assert_eq!(
        result.unwrap_err().message(),
        "`true => false`: published episode should always have a `published_at` date"
    );
}

#[test]
fn assume_implies_format_args_trailing_comma() {
    let field = "published_at";
    let result: Result<(), Assumption> = (|| {
        assume_implies!(
            true => false,
            "published episode should always have a `{}` date",
            field,
        );
        Ok(())
    })();
    assert_eq!(
        result.unwrap_err().message(),
        "`true => false`: published episode should always have a `published_at` date"
    );
}

#[test]
fn assume_implies_arbitrary_error() {
    let result: Result<(), std::io::Error> = (|| {
        assume_implies!(
            true => false,
            std::io::Error::other("consequent should hold when antecedent is true")
        );
        Ok(())
    })();
    assert!(result.is_err());
}

#[test]
fn assume_implies_arbitrary_error_trailing_comma() {
    let result: Result<(), std::io::Error> = (|| {
        assume_implies!(
            true => false,
            std::io::Error::other("consequent should hold when antecedent is true"),
        );
        Ok(())
    })();
    assert!(result.is_err());
}

#[test]
fn assume_implies_with_real_condition() {
    let is_published = std::hint::black_box(true);
    let published_at: Option<u64> = None;
    let result: Result<(), Assumption> = (|| {
        assume_implies!(
            is_published => published_at.is_some(),
            "published episode should always have a publish date"
        );
        Ok(())
    })();
    assert_eq!(
        result.unwrap_err().message(),
        "`is_published => published_at.is_some()`: published episode should always have a publish date"
    );
}

use std::cell::Cell;

fn counted<T: Clone>(counter: &Cell<u32>, value: T) -> T {
    counter.set(counter.get() + 1);
    value
}

#[test]
fn assume_evaluates_condition_once_on_failure() {
    let calls = Cell::new(0);
    let result: Result<(), Assumption> = (|| {
        assume!(counted(&calls, false), "condition should hold");
        Ok(())
    })();
    assert!(result.is_err());
    assert_eq!(calls.get(), 1);
}

#[test]
fn assume_evaluates_condition_once_on_success() {
    let calls = Cell::new(0);
    let result: Result<(), Assumption> = (|| {
        assume!(counted(&calls, true), "condition should hold");
        Ok(())
    })();
    assert!(result.is_ok());
    assert_eq!(calls.get(), 1);
}

#[test]
fn assume_eq_evaluates_each_side_once_on_failure() {
    let (l, r) = (Cell::new(0), Cell::new(0));
    let result: Result<(), Assumption> = (|| {
        assume_eq!(counted(&l, 1), counted(&r, 2), "values should be equal");
        Ok(())
    })();
    assert!(result.is_err());
    assert_eq!((l.get(), r.get()), (1, 1));
}

#[test]
fn assume_ne_evaluates_each_side_once_on_failure() {
    let (l, r) = (Cell::new(0), Cell::new(0));
    let result: Result<(), Assumption> = (|| {
        assume_ne!(counted(&l, 1), counted(&r, 1), "values should differ");
        Ok(())
    })();
    assert!(result.is_err());
    assert_eq!((l.get(), r.get()), (1, 1));
}

#[test]
fn assume_matches_evaluates_expr_once_on_failure_for_every_arm() {
    let calls = Cell::new(0);
    let _: Result<(), Assumption> = (|| {
        assume_matches!(counted(&calls, None::<u32>), Some(_), "should be some");
        Ok(())
    })();
    assert_eq!(calls.get(), 1, "literal arm");

    let calls = Cell::new(0);
    let _: Result<(), Assumption> = (|| {
        assume_matches!(
            counted(&calls, None::<u32>),
            Some(_),
            "should be {}",
            "some"
        );
        Ok(())
    })();
    assert_eq!(calls.get(), 1, "format-args arm");

    let calls = Cell::new(0);
    let _: Result<(), std::io::Error> = (|| {
        assume_matches!(
            counted(&calls, None::<u32>),
            Some(_),
            std::io::Error::other("should be some")
        );
        Ok(())
    })();
    assert_eq!(calls.get(), 1, "error arm");
}

#[test]
fn assume_matches_reports_the_value_that_actually_failed() {
    let mut it = vec![1, 2, 3].into_iter();
    let result: Result<(), Assumption> = (|| {
        assume_matches!(it.next(), Some(5), "first item should be 5");
        Ok(())
    })();
    assert_eq!(
        result.unwrap_err().message(),
        "first item should be 5, got: `Some(1)`"
    );
    assert_eq!(it.collect::<Vec<_>>(), [2, 3]);
}

#[test]
fn assume_matches_evaluates_expr_once_on_success() {
    let calls = Cell::new(0);
    let result: Result<(), Assumption> = (|| {
        assume_matches!(counted(&calls, Some(1)), Some(_), "should be some");
        Ok(())
    })();
    assert!(result.is_ok());
    assert_eq!(calls.get(), 1);
}

#[test]
fn assume_implies_evaluates_operands_once() {
    let (a, b) = (Cell::new(0), Cell::new(0));
    let result: Result<(), Assumption> = (|| {
        assume_implies!(counted(&a, true) => counted(&b, false), "consequent should hold");
        Ok(())
    })();
    assert!(result.is_err());
    assert_eq!((a.get(), b.get()), (1, 1));

    // Consequent must not be evaluated at all when the antecedent is false.
    let (a, b) = (Cell::new(0), Cell::new(0));
    let result: Result<(), Assumption> = (|| {
        assume_implies!(counted(&a, false) => counted(&b, false), "consequent should hold");
        Ok(())
    })();
    assert!(result.is_ok());
    assert_eq!((a.get(), b.get()), (1, 0));
}

#[test]
fn assume_matches_accepts_moving_pattern_on_non_copy_place() {
    let name: Option<String> = Some("tower-of-god".into());
    let result: Result<(), Assumption> = (|| {
        assume_matches!(name, Some(s) if s.len() > 5, "name should be long");
        Ok(())
    })();
    assert!(result.is_ok());
}

#[test]
fn assume_matches_reports_non_copy_value_on_failure() {
    let name: Option<String> = Some("tog".into());
    let result: Result<(), Assumption> = (|| {
        assume_matches!(name, Some(s) if s.len() > 5, "name should be long");
        Ok(())
    })();
    assert_eq!(
        result.unwrap_err().message(),
        "name should be long, got: `Some(\"tog\")`"
    );
}

#[test]
#[deny(unused_variables)]
fn assume_matches_does_not_warn_about_unused_bindings() {
    let result: Result<(), Assumption> = (|| {
        assume_matches!(Some(1u32), Some(n), "should be some");
        Ok(())
    })();
    assert!(result.is_ok());
}

#[test]
fn non_literal_message_bypasses_assumption() {
    const MSG: &str = "episode should have at least one panel";
    let result: Result<(), Box<dyn std::error::Error>> = (|| {
        assume!(std::hint::black_box(false), MSG);
        Ok(())
    })();
    let err = result.unwrap_err();
    assert!(err.downcast_ref::<Assumption>().is_none());
    assert_eq!(err.to_string(), MSG);
}

#[test]
fn assumption_is_send_sync_static() {
    fn check<T: Send + Sync + 'static + std::error::Error>() {}
    check::<Assumption>();
}
