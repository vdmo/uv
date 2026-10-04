; Scopes
[
  (block)
  (procedure_declaration)
  (comptime_procedure_declaration)
  (closure_expression)
  (case_arm)
  (loop_expression)
  (dispatch_expression)
] @local.scope

; Definitions
(parameter name: (identifier) @local.definition.parameter)
(closure_parameter name: (identifier) @local.definition.parameter)
(identifier_pattern (identifier) @local.definition.var)
(typed_pattern (identifier) @local.definition.var)
(field_pattern name: (identifier) @local.definition.var !pattern)
(using_statement alias: (identifier) @local.definition.var)
(region_statement alias: (identifier) @local.definition.var)

; References
(identifier) @local.reference
