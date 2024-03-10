## Grammar
Using Extended-BNF grammar expression, writing out the basic grammar required to express the
program `5 + 8 / 2`. Using javascripts precedence levels, this program should evaluate to `9`.
```
Program = { SourceElement } EOF ;
SourceElement = FunctionDeclaration 
                | Statement ;
Statement = ExpressionStatement ;
ExpressionStatement = Expression [";"] ;
Expression = AdditiveExpression
            | MultiplicativeExpression
            | UnaryExpression ;
AdditiveExpression = MultiplicativeExpression [AdditiveOperator MultiplicativeExpression] ;
MultiplicativeExpression = UnaryExpression [MultiplicativeOperator UnaryExpression] ;
UnaryExpression = Literal ;
Literal = <NUMBER_LITERAL> ;
```

Using this grammar the following AST woul be generated for the simple program `5 + 8 / 2`:
```
Program
    SourceElement
        Statement
            ExpressStatement
                Expression
                    AdditiveExpression
                        lhs
                            MultiplicativeExpression
                                UnaryExpression
                                    Literal 5
                        rhs
                            MultiplicativeExpression
                                lhs
                                    UnaryExpression
                                        Literal 8
                                rhs
                                    UnaryExpression
                                        Literal 2
                                operator
                                    MultiplicativeOperator /
                        AdditiveOperator +
                                    
```
