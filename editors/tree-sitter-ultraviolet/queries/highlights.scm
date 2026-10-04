; Ultraviolet highlights. Later patterns take priority in editors that use last-match
; (Neovim, Zed), so general captures come first.

; ---- identifiers --------------------------------------------------------

(identifier) @variable

((identifier) @variable.builtin
  (#eq? @variable.builtin "self"))

((identifier) @constant
  (#match? @constant "^[A-Z][A-Z0-9_]+$"))

; ---- types --------------------------------------------------------------

(type_path (identifier) @type)
(generic_name (identifier) @type)
(class_bound (type_path (identifier) @type))

((type_path (identifier) @type.builtin)
  (#match? @type.builtin "^(i8|i16|i32|i64|i128|u8|u16|u32|u64|u128|isize|usize|f16|f32|f64|bool|char|string|bytes|Ptr|Self|Type)$"))

(state_type state: (identifier) @constructor)
(state_name state: (identifier) @constructor)
(never_type) @type.builtin
(unit_type) @type.builtin

(record_declaration name: (identifier) @type)
(enum_declaration name: (identifier) @type)
(modal_declaration name: (identifier) @type)
(class_declaration name: (identifier) @type)
(type_alias_declaration name: (identifier) @type)
(associated_type name: (identifier) @type)
(generic_parameter name: (identifier) @type)

(record_literal type: (identifier) @type)
(record_literal type: (scoped_identifier name: (identifier) @type))
(record_pattern type: (type_path (identifier) @type))

; ---- declarations -------------------------------------------------------

(procedure_declaration name: (identifier) @function)
(procedure_signature name: (identifier) @function)
(comptime_procedure_declaration name: (identifier) @function)
(transition_declaration name: (identifier) @function.method)
(transition_declaration target: (identifier) @constructor)
(derive_target_declaration name: (identifier) @function.macro)
(derive_clause (identifier) @function.macro)

(parameter name: (identifier) @variable.parameter)
(closure_parameter name: (identifier) @variable.parameter)
(receiver) @variable.builtin

(field_declaration name: (identifier) @property)
(field_initializer name: (identifier) @property)
(field_pattern name: (identifier) @property)
(field_expression field: (identifier) @property)
(field_expression field: (integer_literal) @property)

(variant name: (identifier) @constructor)
(state_block name: (identifier) @constructor)
(abstract_state name: (identifier) @constructor)
(modal_pattern state: (identifier) @constructor)
(modal_literal state: (identifier) @constructor)
(enum_literal variant: (identifier) @constructor)
(enum_pattern (identifier) @constructor .)

(import_declaration path: (module_path (identifier) @module))
(import_declaration alias: (identifier) @module)
(scoped_identifier path: (identifier) @module)
(using_specifier name: (identifier) @variable)

; ---- calls --------------------------------------------------------------

(call_expression function: (identifier) @function.call)
(call_expression function: (scoped_identifier name: (identifier) @function.call))
(call_expression function: (field_expression field: (identifier) @function.method.call))
(generic_call function: (identifier) @function.call)
(generic_call function: (scoped_identifier name: (identifier) @function.call))
(method_call_expression name: (identifier) @function.method.call)

((call_expression function: (scoped_identifier name: (identifier) @constructor))
  (#match? @constructor "^[A-Z]"))

; ---- attributes and contracts --------------------------------------------

(attribute "#" @attribute)
(attribute name: (attribute_name) @attribute)
(attribute_argument key: (identifier) @property)
(contract_intrinsic "@" @function.builtin name: (identifier) @function.builtin)
(foreign_contract "@" @function.builtin kind: (identifier) @function.builtin)
(key_boundary) @punctuation.special
(option name: (identifier) @property)

; ---- literals -----------------------------------------------------------

(integer_literal) @number
(float_literal) @number.float
(string_literal) @string
(char_literal) @character
(escape_sequence) @string.escape
(boolean_literal) @boolean
(null_literal) @constant.builtin
(unit) @constant.builtin
(wildcard_pattern) @variable.builtin

; ---- comments -----------------------------------------------------------

(line_comment) @comment
(block_comment) @comment
(doc_comment) @comment.documentation

; ---- keywords -----------------------------------------------------------

[
  "let" "var" "type" "record" "enum" "modal" "class" "extern" "derive" "target"
  "comptime" "quote" "transition" "override" "region" "frame" "defer" "unsafe"
  "as" "is" "in" "using" "pattern"
] @keyword

"procedure" @keyword.function
"import" @keyword.import

[ "public" "internal" "private" "protected" ] @keyword.modifier
[ "const" "unique" "shared" "imm" "mut" "move" "copy" "widen" "new" "opaque" ] @keyword.modifier

[ "if" "else" ] @keyword.conditional
[ "loop" "break" ] @keyword.repeat
(continue_statement) @keyword.repeat
"return" @keyword.return

[
  "spawn" "parallel" "dispatch" "race" "all" "sync" "wait" "yield" "from" "release" "key"
] @keyword.coroutine

[ "transmute" "sizeof" "alignof" ] @function.builtin

(key_block_statement "%" @keyword)
(key_block_statement [ "read" "write" "release" "speculative" ] @keyword)
(key_clause [ "read" "write" ] @keyword)
(derive_clause [ "emits" "requires" ] @keyword)

; ---- operators and punctuation -------------------------------------------

[
  "+" "-" "*" "/" "%" "**" "==" "!=" "<" "<=" ">" ">=" "&&" "||" "!" "&" "|" "^" "<<" ">>"
  "=" ":=" "+=" "-=" "*=" "/=" "%=" "&=" "|=" "^=" "<<=" ">>=" ".." "..=" "=>" "->" "~>"
  "?" "|:" "<:" "$" "@"
] @operator

[ "(" ")" "[" "]" "{" "}" ] @punctuation.bracket
[ "," ";" ":" "::" "." ] @punctuation.delimiter
