use jaq_core::{parse, Ctx, Definitions, Error, RcIter, Val};
use serde_json::{json, Value};
use std::str::FromStr;

#[test]
fn test() {
    println!("{}", "hello world");
    println!("{}", serde_json::Value::from("hello world"));
}

#[test]
fn jq_test() {
    let input = r#"["Hello", "world"]"#;
    let filter = ".[]";
    let out = jq(input, filter);
    let mut out = out.into_iter();

    assert_eq!(out.next(), Some(Ok(Val::from(json!("Hello")))));
    assert_eq!(out.next(), Some(Ok(Val::from(json!("world")))));
    assert_eq!(out.next(), None);
}

#[test]
fn jq_test_2() {
    let input = r#"{ "message": "hello world" }"#;
    let filter = ".message";
    let out = jq(input, filter);
    let mut out = out.into_iter();

    assert_eq!(out.next(), Some(Ok(Val::from(json!("hello world")))));
    assert_eq!(out.next(), None);
}

fn jq(input: &str, filter: &str) -> Vec<Result<Val, Error>> {
    let input = Value::from_str(input).unwrap();

    // start out only from core filters,
    // which do not include filters in the standard library
    // such as `map`, `select` etc.
    let defs = Definitions::core();

    // parse the filter in the context of the given definitions
    let mut errs = Vec::new();
    let f = parse::parse(&filter, parse::main()).0.unwrap();
    let f = defs.finish(f, Vec::new(), &mut errs);

    //assert_eq!(errs, Vec::new());
    let inputs = RcIter::new(core::iter::empty());

    // iterator over the output values
    let out = f.run(Ctx::new([], &inputs), Val::from(input));

    out.collect()
}

#[test]
fn v2() {
    use evalexpr::*;
    let context = r#"
        a = 1
    "#;

    let output = evalexpr::eval(context);
    let mut context = HashMapContext::new();
    context.set_value("user.id".into(), 1.into()).unwrap(); // Do proper error handling here
    context.set_value("two".into(), 2.into()).unwrap(); // Do proper error handling here
    context.set_value("three".into(), 3.into()).unwrap(); // Do proper error handling here
    assert_eq!(
        eval_with_context("user.id + two + three", &context),
        Ok(Value::from(6))
    );
    assert_eq!(eval_with_context("user.id", &context), Ok(Value::from(1)));
    println!("hello world {:#?}", output);
}
