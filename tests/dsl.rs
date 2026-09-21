use sphered::eval;

#[test]
fn lexes_every_operator() {
    let toks = sphered::lexer::lex("( ) < > ? ! | @ # ~").unwrap();
    use sphered::lexer::Token::*;
    assert_eq!(
        toks,
        vec![LParen, RParen, LAngle, RAngle, Question, Bang, Pipe, At, Hash, Tilde]
    );
}

#[test]
fn atom_and_nested_sphere() {
    let out = eval("(a (b c) d)").unwrap();
    assert_eq!(out.value, "d");
    assert_eq!(
        out.trace,
        vec![
            "follow (".to_string(),
            "follow (".to_string(),
            "return c".to_string(),
            "return d".to_string(),
        ]
    );
}

#[test]
fn inner_evaluates_before_outer() {
    // the inner sphere's return appears before the outer's return
    let out = eval("(x (y))").unwrap();
    let follow_pos = out.trace.iter().position(|l| l == "follow (").unwrap();
    let inner_return = out.trace.iter().position(|l| l == "return y").unwrap();
    let outer_return = out.trace.iter().position(|l| l == "return y").unwrap();
    assert!(follow_pos < inner_return);
    assert_eq!(inner_return, outer_return); // same value bubbles up
}

#[test]
fn transaction_emits_a_witness() {
    let out = eval("<( event ? admissible ! valid @ sensor > committed )>").unwrap();
    assert_eq!(out.value, "committed");
    assert!(out.trace.contains(&"admit admissible".to_string()));
    assert!(out.trace.contains(&"verify valid".to_string()));
    assert!(out.trace.contains(&"witness sensor".to_string()));
    assert!(out.trace.contains(&"commit committed".to_string()));
    assert!(out.trace.last().unwrap().starts_with("witnessed 1 sources"));
}

#[test]
fn nested_transaction_follows_inward() {
    let out = eval("(outer <( inner ~ normalize ! valid )> )").unwrap();
    assert_eq!(out.value, "valid");
    assert!(out.trace.contains(&"transform normalize".to_string()));
    assert!(out.trace.contains(&"verify valid".to_string()));
}

#[test]
fn unclosed_sphere_is_an_error() {
    assert!(eval("(a b").is_err());
}
