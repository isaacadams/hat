pub enum BinaryOperator {
    Equal,
    GreaterThan,
    LessThan,
    GreaterThanOrEqualTo,
    LessThanOrEqualTo,
}

impl BinaryOperator {
    pub fn as_str(&self) -> &'static str {
        match &self {
            BinaryOperator::Equal => "==",
            BinaryOperator::GreaterThan => ">",
            BinaryOperator::LessThan => "<",
            BinaryOperator::GreaterThanOrEqualTo => ">=",
            BinaryOperator::LessThanOrEqualTo => "<=",
        }
    }

    pub fn from_str(operator: &str) -> Option<Self> {
        match operator {
            "==" => Some(BinaryOperator::Equal),
            ">" => Some(BinaryOperator::GreaterThan),
            "<" => Some(BinaryOperator::LessThan),
            ">=" => Some(BinaryOperator::GreaterThanOrEqualTo),
            "<=" => Some(BinaryOperator::LessThanOrEqualTo),
            _ => None,
        }
    }

    pub fn print<T: AsRef<str>, Q: AsRef<str>>(&self, left: T, right: Q) -> String {
        format!("{} {} {}", left.as_ref(), self.as_str(), right.as_ref())
    }

    pub fn execute<T: PartialEq<Q> + PartialOrd<Q>, Q: PartialEq<T> + PartialOrd<T>>(
        &self,
        left: T,
        right: Q,
    ) -> bool {
        match &self {
            BinaryOperator::Equal => left == right,
            BinaryOperator::GreaterThan => left > right,
            BinaryOperator::LessThan => left < right,
            BinaryOperator::GreaterThanOrEqualTo => left >= right,
            BinaryOperator::LessThanOrEqualTo => left <= right,
        }
    }
}
