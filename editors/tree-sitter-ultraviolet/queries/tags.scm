(procedure_declaration name: (identifier) @name) @definition.function
(comptime_procedure_declaration name: (identifier) @name) @definition.function
(procedure_signature name: (identifier) @name) @definition.function
(transition_declaration name: (identifier) @name) @definition.method
(record_declaration name: (identifier) @name) @definition.class
(enum_declaration name: (identifier) @name) @definition.class
(modal_declaration name: (identifier) @name) @definition.class
(class_declaration name: (identifier) @name) @definition.interface
(type_alias_declaration name: (identifier) @name) @definition.type
(variant name: (identifier) @name) @definition.constant
(field_declaration name: (identifier) @name) @definition.field

(call_expression function: (identifier) @name) @reference.call
(call_expression function: (scoped_identifier name: (identifier) @name)) @reference.call
(method_call_expression name: (identifier) @name) @reference.call
