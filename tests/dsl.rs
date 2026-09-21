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
    let out = eval("(outer <( inner ~ normalize ! valid @ src > done )> )").unwrap();
    assert_eq!(out.value, "done");
    assert!(out.trace.contains(&"transform normalize".to_string()));
    assert!(out.trace.contains(&"verify valid".to_string()));
    assert!(out.trace.contains(&"witness src".to_string()));
    assert!(out.trace.contains(&"commit done".to_string()));
}

#[test]
fn unclosed_sphere_is_an_error() {
    assert!(eval("(a b").is_err());
}

#[test]
fn admission_after_transform_is_rejected() {
    let e = eval("<( req ~ write ? allowed ! chk @ src > done )>");
    assert!(e.is_err());
    assert!(e.unwrap_err().contains("admission after transformation"));
}

#[test]
fn verification_after_commit_is_rejected() {
    let e = eval("<( req ? allowed ~ write @ src > done ! chk )>");
    assert!(e.is_err());
}

#[test]
fn no_verification_is_rejected() {
    let e = eval("<( req ? allowed ~ write @ src > done )>");
    assert!(e.is_err());
    assert!(e.unwrap_err().contains("no verification clause"));
}

#[test]
fn two_commits_are_rejected() {
    let e = eval("<( req ? allowed ~ f ! chk @ src > c1 > c2 )>");
    assert!(e.is_err());
}

#[test]
fn admit_refusal_stops_the_frame() {
    let out = eval("<( event ? x ~ process ! chk @ src > done )>").unwrap();
    assert_eq!(out.value, "refuse");
    assert!(out.trace.contains(&"refuse admit".to_string()));
    assert!(out.trace.contains(&"refused, no witness".to_string()));
    assert!(!out.trace.iter().any(|l| l == "commit done"));
}

#[test]
fn verify_refusal_stops_the_frame() {
    let out = eval("<( event ? allowed ~ process ! x @ src > done )>").unwrap();
    assert_eq!(out.value, "refuse");
    assert!(out.trace.contains(&"refuse verify".to_string()));
    assert!(!out.trace.iter().any(|l| l == "commit done"));
}

#[test]
fn admission_leaves_no_witness() {
    let out = eval("<( event ? allowed ~ process ! chk @ src > done )>").unwrap();
    assert_eq!(out.value, "done");
    assert!(out
        .trace
        .contains(&"witnessed 1 sources, exit >".to_string()));
}
