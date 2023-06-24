use crate::hat_util::Assert;

pub fn is_true<T: AsRef<str>>(expression: T) -> bool {
    use evalexpr::{eval, Value};
    eval(expression.as_ref()) == Ok(Value::Boolean(true))
}

pub struct TestAssertions {
    name: String,
    assertions: String,
}

impl TestAssertions {
    fn new(name: String, assertions: String) -> Self {
        Self { name, assertions }
    }
}

pub fn new(name: String, assertions: String) -> TestAssertions {
    TestAssertions::new(name, assertions)
}

pub fn pretty_bool(result: bool) -> &'static str {
    if result {
        "✅ "
    } else {
        "❌ "
    }
}

impl Assert for TestAssertions {
    fn assert(&self, buffer: &mut String) -> bool {
        let mut test = true;
        let start = buffer.len();
        buffer.push_str(&self.name);

        for t in self.assertions.lines() {
            let assertion = t;
            let test_result = self::is_true(assertion);
            test &= test_result;
            buffer.push_str("\n  ");
            buffer.push_str(self::pretty_bool(test_result));
            buffer.push_str(assertion);
        }

        buffer.insert_str(start, self::pretty_bool(test));
        buffer.insert_str(start, "\n\n");

        test
    }
}
