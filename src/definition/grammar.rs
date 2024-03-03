// Operator is more of a grammar thing
// we should leave that out of token and go into
// parsing 
#[derive(Debug)]
pub enum Operator {
    Delete,
    Void,
    Typeof,
    Plus, /* converts to Number, not addition */
    Minus, /* converts to Negative Number, not subtraction */
    BitwiseNot,
    LogicalNot,
    Await,
    Exponential,
    Multiply,
    Add,
    Subtract,
    Divide,
    Remainder,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    Instanceof,
    Equality,
    Inequality,
    StrictEquality,
    StrictInequality,
    BitwiseLeftShift,
    BitwiseRightShift,
    BitwiseUnsignedRightShift,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LogicalAnd,
    LogicalOr,
    NullishCoalesce,
    Assignment,
    AssignmentMultiply,
    AssignmentDivide,
    AssignmentRemainder,
    AssignmentAdd,
    AssignmentSubtract,
    AssignmentLeftShift,
    AssignmentRightShift,
    AssignmentUnsignedRightShift,
    AssignmentBitwiseAnd,
    AssignmentBitwiseOr,
    AssignmentBitwiseXor,
    AssignmentExponate,
    AssignmentLogicalAnd,
    AssignmentLogicalOr,
    AssignmentNullishCoalesce,
    Yield,
    Comma,
    Spread,
}

