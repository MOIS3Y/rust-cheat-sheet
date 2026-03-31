//! Conditionals — `if`, `else`, and `match` expressions.
//!
//! Demonstrates conditional expressions, pattern matching,
//! and control flow based on boolean conditions.

/// Simple if statement.
///
/// `if` evaluates code based on boolean condition.
pub fn if_simple() {
    let n = 10;
    let mut is_positive = false;
    if n > 0 {
        is_positive = true;
    }
    assert!(is_positive);
}

/// If-else expression.
///
/// `if` can return values in both branches.
pub fn if_else_value() {
    let n = -5;
    let abs = if n < 0 { -n } else { n };
    assert_eq!(abs, 5);
}

/// Else-if chain.
///
/// Multiple conditions can be chained with `else if`.
pub fn else_if_chain() {
    let n = 0;
    let sign = if n > 0 {
        "positive"
    } else if n < 0 {
        "negative"
    } else {
        "zero"
    };
    assert_eq!(sign, "zero");
}

/// If with multiple conditions.
///
/// Conditions can be combined with `&&` and `||`.
pub fn if_multiple_conditions() {
    let n = 5;
    let in_range = n >= 0 && n < 10;
    assert!(in_range);
}

/// If let for pattern matching.
///
/// `if let` matches single pattern concisely.
pub fn if_let_option() {
    let opt = Some(42);
    if let Some(value) = opt {
        assert_eq!(value, 42);
    }
}

/// If let with else.
///
/// Provides fallback for None case.
pub fn if_let_with_else() {
    let opt: Option<i32> = None;
    let value = if let Some(v) = opt {
        v
    } else {
        -1
    };
    assert_eq!(value, -1);
}

/// Match expression basic.
///
/// `match` compares value against patterns.
pub fn match_basic() {
    let n = 2;
    let desc = match n {
        0 => "zero",
        1 => "one",
        2 => "two",
        _ => "other",
    };
    assert_eq!(desc, "two");
}

/// Match with multiple patterns.
///
/// Multiple patterns use `|` separator.
pub fn match_multiple_patterns() {
    let day = 7;
    let is_weekend = match day {
        6 | 7 => true,
        _ => false,
    };
    assert!(is_weekend);
}

/// Match with guards.
///
/// `if` condition after pattern refines matching.
pub fn match_with_guards() {
    let n = 15;
    let desc = match n {
        n if n < 10 => "small",
        n if n < 20 => "medium",
        _ => "large",
    };
    assert_eq!(desc, "medium");
}

/// Match destructuring tuple.
///
/// Patterns can destructure compound types.
pub fn match_destructure_tuple() {
    let point = (0, 5);
    let desc = match point {
        (0, 0) => "origin",
        (0, _) => "on y-axis",
        (_, 0) => "on x-axis",
        _ => "somewhere",
    };
    assert_eq!(desc, "on y-axis");
}

/// Matches macro for boolean.
///
/// `matches!` returns true if pattern matches.
pub fn matches_macro() {
    let opt = Some(42);
    assert!(matches!(opt, Some(_)));
    assert!(!matches!(opt, None));
}

/// Matches with guard.
///
/// Guard condition in matches! macro.
pub fn matches_with_guard() {
    let opt = Some(42);
    assert!(matches!(opt, Some(v) if v > 40));
    assert!(!matches!(opt, Some(v) if v > 50));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_if_simple() {
        if_simple();
    }

    #[test]
    fn test_if_else_value() {
        if_else_value();
    }

    #[test]
    fn test_else_if_chain() {
        else_if_chain();
    }

    #[test]
    fn test_if_multiple_conditions() {
        if_multiple_conditions();
    }

    #[test]
    fn test_if_let_option() {
        if_let_option();
    }

    #[test]
    fn test_if_let_with_else() {
        if_let_with_else();
    }

    #[test]
    fn test_match_basic() {
        match_basic();
    }

    #[test]
    fn test_match_multiple_patterns() {
        match_multiple_patterns();
    }

    #[test]
    fn test_match_with_guards() {
        match_with_guards();
    }

    #[test]
    fn test_match_destructure_tuple() {
        match_destructure_tuple();
    }

    #[test]
    fn test_matches_macro() {
        matches_macro();
    }

    #[test]
    fn test_matches_with_guard() {
        matches_with_guard();
    }
}
