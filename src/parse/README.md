## Grammar
```
Program = { SourceElement } EOF ;
SourceElement = FunctionDeclaration 
                | Statement ;
Statement = Block 
            | ExpressionStatement
            | IfStatement
            ;
Block = "{" [StatementList] "}" ;
StatementList = { Statement } ;
ExpressionStatement = Expression [";"] ;
IfStatement = "if" "(" Expression ")" Statement ["else" Statement] ;
Expression = AdditiveExpression
            | MultiplicativeExpression
            | UnaryExpression ;
AdditiveExpression = MultiplicativeExpression [AdditiveOperator MultiplicativeExpression] ;
MultiplicativeExpression = UnaryExpression [MultiplicativeOperator UnaryExpression] ;
MultiplicativeOperator = "*" | "/" | "%" ;
UnaryExpression = Literal ;
UnaryOperator = "delete" | "void" | "typeof"
    | "++" | "--" | "+" | "-" | "~" | "!" ;
Literal = <NUMBER_LITERAL> ;
```
