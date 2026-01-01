(comment) @comment

(string) @string
(interpolated_string_content_double) @string
(interpolated_string_content_single) @string
(interpolated_string_content_backtick) @string

(number) @number
(boolean) @boolean
(nil) @boolean
(self) @type
(print) @function
(builtin_type) @type
(constructor) @keyword
(identifier) @variable
(all_caps_identifier) @constant
(escape_sequence) @string.escape

"$" @constant
"@" @keyword
"\"" @string
"'" @string
"`" @string

(decorator_statement
	(identifier) @type)
(decorator_statement
	"@" @type)

[
	"let"
	"const"
	"fn"
	"loop"
	"while"
	"for"
	"using"
	"class"
	"namespace"
	"if"
	"else"
	"switch"
	"return"
	"break"
	"out"
	"continue"
	"=>"
	"in"
] @keyword

(pascal_case_identifier) @type

(function_declaration
  (snake_case_identifier) @function)

(call_expression
  name: (identifier) @function)

(DOT_DOT) @number
