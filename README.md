순천대학교 컴퓨터교육과 주한결

TSMC언어 마노 베이직 컴퓨터 컴파일러

---

TSMC 언어 EBNF

<program> ::= [<functions>]

<functions> ::= <function> [<functions>]

<function> ::= <function declaration> <compound_statement>

<function_declaration> ::=  <declarator> “(” [<params>] “)”

<params> ::= [ <declarator> [ “,” <params> ] ]

<declaration> ::= <declarator> “;”

<declarator> ::= <type> <id>

<statement> ::= <simple_statement> | <control_statement> |<compound_statement> | <return_statement>

<expression_statement> ::=[<assign_expr>] “;”

<return_statement> ::= <return> “;”

<control_statement> ::= <if_stmt>  | <while_stmt>

<compound_statement> ::= “{”[<declarations>] [<statements>]”}”

<declarations> ::= <declaration> [<declarations>]

<statements> ::= <statement> [<statements>]

<procedure_call> ::= <id> “(” [<call_exprs>] “)”

<call_exprs> ::= <assign_expr> [“,” <call_exprs>]

<if_stmt> ::= “if” “(” <assign_expr> “)” <compound_statement>

<while_stmt> ::= “while” “(” <assign_expr> “)” <compound_statement>

<return_statement> ::= “return” <assign_expr> ;

<assign_expr> :: <condition_expr> [“=” <assign_expr> ]

<condition_expr> ::= <additive_expr> [ “==” <condition_expr> ]

<additive_expr> ::= <term> [((“+” | “-”) <additive_expr> )]

<term> ::= <unary_expr> [((“*” | “/”) <term>)]

<unary_expr> ::=  [ “&” | “*” | “+” | “-”] <unary_expr> | <factor>

<factor> ::= <literal> | <id> | “(” <assign_expr> “)” | <procedure_call> | “(“ <type>“)” <factor>

<procedure_call> ::= <id> ‘(‘ <arg_list> ‘)’

<arg_list> ::= <assign_expr> [‘,’  <arg_list>]

<literal> ::= <natural_numbers>

<type> ::=  <basic_type> <stars>

<stars> ::= [“*” [<stars>]]

<basic_type> ::= “int” 
