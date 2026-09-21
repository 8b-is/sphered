use crate::ast::{Expr, Step};

#[derive(Debug, Clone, PartialEq)]
pub struct Output {
    pub value: String,
    pub trace: Vec<String>,
}

pub fn eval(source: &str) -> Result<Output, String> {
    let tokens = crate::lexer::lex(source)?;
    let expr = crate::parser::parse(&tokens)?;
    let mut trace = Vec::new();
    let value = eval_expr(&expr, &mut trace);
    Ok(Output { value, trace })
}

fn eval_expr(e: &Expr, trace: &mut Vec<String>) -> String {
    match e {
        Expr::Atom(s) => s.clone(),
        Expr::Sphere(exprs) => {
            trace.push("follow (".to_string());
            let mut last = String::new();
            for x in exprs {
                last = eval_expr(x, trace);
            }
            trace.push(format!("return {last}"));
            last
        }
        Expr::Txn(steps) => {
            trace.push("enter <(".to_string());
            let mut current = String::new();
            let mut witnessed = Vec::new();
            for s in steps {
                match s {
                    Step::Plain(x) => {
                        current = eval_expr(x, trace);
                    }
                    Step::Admit(x) => {
                        let v = eval_expr(x, trace);
                        trace.push(format!("admit {v}"));
                    }
                    Step::Bind(x) => {
                        let v = eval_expr(x, trace);
                        trace.push(format!("bind {v}"));
                    }
                    Step::Transform(x) => {
                        let v = eval_expr(x, trace);
                        current = v.clone();
                        trace.push(format!("transform {v}"));
                    }
                    Step::Verify(x) => {
                        let v = eval_expr(x, trace);
                        current = v.clone();
                        trace.push(format!("verify {v}"));
                    }
                    Step::Witness(x) => {
                        let v = eval_expr(x, trace);
                        witnessed.push(v.clone());
                        trace.push(format!("witness {v}"));
                    }
                    Step::Remember(x) => {
                        let v = eval_expr(x, trace);
                        trace.push(format!("remember {v}"));
                    }
                    Step::Commit(x) => {
                        current = eval_expr(x, trace);
                        trace.push(format!("commit {current}"));
                    }
                }
            }
            trace.push(format!("witnessed {} sources, exit >", witnessed.len()));
            current
        }
    }
}
