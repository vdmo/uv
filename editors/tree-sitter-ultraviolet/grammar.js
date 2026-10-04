/**
 * Tree-sitter grammar for Ultraviolet.
 *
 * Follows Appendix B of the language specification and the reference parser. The grammar
 * is for editors: it accepts everything the compiler's parser accepts and is lenient
 * where strictness would only hurt incremental parsing.
 *
 * Newlines terminate statements. The external scanner (src/scanner.c) produces a
 * terminator only where the parse state can take one and the next token does not continue
 * the line, which is the reference lexer's newline filter.
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

const PREC = {
  range: 1,
  or: 2,
  and: 3,
  compare: 4,
  bit_or: 5,
  bit_xor: 6,
  bit_and: 7,
  shift: 8,
  add: 9,
  mul: 10,
  pow: 11,
  cast: 12,
  unary: 13,
  pipeline: 14,
  postfix: 15,
  path: 16,
};

const INT_SUFFIX = /(i8|i16|i32|i64|i128|u8|u16|u32|u64|u128|isize|usize)?/;
const DEC = /[0-9](_*[0-9])*/;

const commaSep1 = rule => seq(rule, repeat(seq(',', rule)), optional(','));
const commaSep = rule => optional(commaSep1(rule));
const declPrefix = $ => [optional($.visibility)];

module.exports = grammar({
  name: 'ultraviolet',

  externals: $ => [$._newline, $.block_comment, $._option_bracket, $._generic_open, $._error_sentinel],

  extras: $ => [/[\s﻿]/, $.line_comment, $.doc_comment, $.block_comment],

  word: $ => $.identifier,

  supertypes: $ => [$._item, $._statement, $._expression, $._pattern, $._type],

  conflicts: $ => [
    [$._expression, $._name],
    [$._expression, $._pattern],
    [$._expression, $.generic_name],
    [$._expression, $.identifier_pattern],
    [$._expression, $.labeled_frame],
    [$._expression, $.loop_expression],
    [$._expression, $.record_literal, $._name],
    [$._expression, $.record_literal],
    [$._expression, $.state_name],
    [$._pattern, $._name],
    [$._type_no_union, $.generic_type],
    [$._type_no_union, $.state_type],
    [$.case_block],
    [$.contract_intrinsic, $._name],
    [$.field_pattern, $.field_initializer],
    [$.foreign_contract, $.contract_intrinsic],
    [$.generic_parameter],
    [$.identifier_pattern, $._name],
    [$.key_boundary, $.attribute],
    [$.literal_pattern, $._expression],
    [$.literal_pattern, $._literal],
    [$.modal_pattern, $.field_initializer_list],
    [$.modal_pattern],
    [$.predicate_block, $.expression_statement],
    [$.procedure_declaration, $.procedure_signature],
    [$.procedure_signature],
    [$.record_literal, $._name],
    [$.splice, $._name],
    [$.tuple_type, $.parameter_types],
    [$.type_path, $.enum_pattern],
    [$.unit, $.tuple_pattern],
    [$.unit_type, $.parameter_types],
    [$.using_declaration, $.using_statement],
    [$.wildcard_pattern, $.typed_pattern],
  ],

  rules: {
    source_file: $ => repeat(choice($._item, $.attribute, $._terminator)),

    _terminator: $ => choice(';', $._newline),

    // ---- comments --------------------------------------------------------

    line_comment: _ => token(seq('//', /[^\n]*/)),
    doc_comment: _ => token(prec(1, seq(choice('///', '//!'), /[^\n]*/))),

    // ---- items -----------------------------------------------------------

    _item: $ => choice(
      $.import_declaration,
      $.using_declaration,
      $.static_declaration,
      $.procedure_declaration,
      $.comptime_procedure_declaration,
      $.record_declaration,
      $.enum_declaration,
      $.modal_declaration,
      $.class_declaration,
      $.type_alias_declaration,
      $.extern_block,
      $.derive_target_declaration,
    ),

    visibility: _ => choice('public', 'internal', 'private', 'protected'),

    import_declaration: $ => seq(
      ...declPrefix($), 'import', field('path', $.module_path),
      optional(seq('as', field('alias', $.identifier))),
    ),

    using_declaration: $ => seq(
      ...declPrefix($), 'using', $._name, repeat(seq('::', $._name)),
      optional(choice(seq('::', '*'), seq('::', $.using_list))),
      optional(seq('as', field('alias', $._name))),
    ),
    using_list: $ => seq('{', commaSep($.using_specifier), '}'),
    using_specifier: $ => seq(
      field('name', $.identifier), optional(seq('as', field('alias', $.identifier))),
    ),
    module_path: $ => prec.left(seq($._name, repeat(seq('::', $._name)))),

    static_declaration: $ => seq(
      ...declPrefix($), choice('let', 'var'), field('pattern', $._pattern),
      optional(seq(':', field('type', $._type))),
      choice('=', ':='), field('value', $._value),
    ),

    procedure_declaration: $ => seq(
      ...declPrefix($), optional('override'), 'procedure', field('name', $._name),
      optional(field('type_parameters', $.generic_parameters)),
      field('parameters', $.parameters),
      optional(seq('->', field('return_type', $._type))),
      optional(seq(optional($._nl), $.contract_clause)),
      optional($._nl),
      field('body', $.block),
    ),

    comptime_procedure_declaration: $ => seq(
      'comptime', optional($.visibility), 'procedure',
      field('name', $._name),
      optional(field('type_parameters', $.generic_parameters)),
      field('parameters', $.parameters),
      optional(seq('->', field('return_type', $._type))),
      optional(seq(optional($._nl), $.contract_clause)),
      optional($._nl),
      field('body', $.block),
    ),

    _nl: $ => repeat1($._newline),

    parameters: $ => seq('(', commaSep(choice($.parameter, $.receiver)), ')'),
    parameter: $ => seq(
      optional('move'), field('name', $._name), ':', field('type', $._type),
    ),
    receiver: _ => choice('~', '~!', '~%'),

    generic_parameters: $ => seq(
      '<', $.generic_parameter, repeat(seq(choice(';', ','), $.generic_parameter)), optional(';'), '>',
    ),
    generic_parameter: $ => seq(
      optional(field('variance', choice('+', '-', '='))),
      field('name', $._name),
      optional(seq('<:', $.class_bound, repeat(seq(choice(',', '+'), $.class_bound)))),
      optional(seq('=', field('default', $._type))),
    ),
    class_bound: $ => seq($.type_path, optional($.generic_arguments)),
    generic_arguments: $ => seq('<', commaSep($._type), '>'),

    contract_clause: $ => prec.right(seq(
      '|:',
      choice(
        seq(field('precondition', $._contract_predicate),
          optional(seq('|=', field('postcondition', $._contract_predicate)))),
        seq('|=', field('postcondition', $._contract_predicate)),
      ),
    )),
    _contract_predicate: $ => choice($._value, $.predicate_block),
    predicate_block: $ => seq('{', $._expression, '}'),

    type_invariant: $ => seq('|:', '{', $._expression, '}'),

    implements_clause: $ => seq('<:', $.class_bound, repeat(seq(choice(',', '+'), $.class_bound))),

    record_declaration: $ => seq(
      ...declPrefix($), 'record', field('name', $._name),
      optional(field('type_parameters', $.generic_parameters)),
      optional($.implements_clause),
      optional($._nl),
      field('body', $.record_body),
      optional($.type_invariant),
    ),
    record_body: $ => seq('{', repeat(choice($._record_member, $._terminator, ',', $.attribute)), '}'),
    _record_member: $ => choice($.field_declaration, $.procedure_declaration, $.associated_type),
    field_declaration: $ => seq(
      ...declPrefix($), optional($.key_boundary), field('name', $._name), ':', field('type', $._type),
      optional(seq('=', field('default', $._expression))),
    ),
    key_boundary: _ => '#',
    associated_type: $ => seq(
      ...declPrefix($), 'type', field('name', $._name), optional(seq('=', field('type', $._type))),
    ),

    enum_declaration: $ => seq(
      ...declPrefix($), 'enum', field('name', $._name),
      optional(field('type_parameters', $.generic_parameters)),
      optional($.implements_clause),
      optional($._nl),
      field('body', $.enum_body),
      optional($.type_invariant),
    ),
    enum_body: $ => seq('{', repeat(choice($.variant, $._terminator, ',', $.attribute)), '}'),
    variant: $ => seq(
      field('name', $._name),
      optional(choice($.variant_tuple_payload, $.variant_record_payload)),
      optional(seq('=', field('discriminant', $.integer_literal))),
    ),
    variant_tuple_payload: $ => seq('(', commaSep($._type), ')'),
    variant_record_payload: $ => seq('{', commaSep($.field_declaration), '}'),

    modal_declaration: $ => seq(
      ...declPrefix($), 'modal', field('name', $._name),
      optional(field('type_parameters', $.generic_parameters)),
      optional($.implements_clause),
      optional($._nl),
      field('body', $.modal_body),
      optional($.type_invariant),
    ),
    modal_body: $ => seq('{', repeat(choice($.state_block, $._terminator, $.attribute)), '}'),
    state_block: $ => seq(
      '@', field('name', $._name),
      '{', repeat(choice($._state_member, $._terminator, ',', $.attribute)), '}',
    ),
    _state_member: $ => choice($.field_declaration, $.procedure_declaration, $.transition_declaration),
    transition_declaration: $ => seq(
      ...declPrefix($), 'transition', field('name', $._name), field('parameters', $.parameters),
      '->', '@', field('target', $._name), field('body', $.block),
    ),

    class_declaration: $ => seq(
      ...declPrefix($), optional('modal'), 'class', field('name', $._name),
      optional(field('type_parameters', $.generic_parameters)),
      optional($.implements_clause),
      optional($._nl),
      field('body', $.class_body),
    ),
    class_body: $ => seq('{', repeat(choice($._class_item, $._terminator, $.attribute)), '}'),
    _class_item: $ => choice(
      $.field_declaration, $.procedure_declaration, $.procedure_signature, $.associated_type,
      $.abstract_state,
    ),
    procedure_signature: $ => seq(
      ...declPrefix($), optional('override'), 'procedure', field('name', $._name),
      optional(field('type_parameters', $.generic_parameters)),
      field('parameters', $.parameters),
      optional(seq('->', field('return_type', $._type))),
      optional(seq(optional($._nl), $.contract_clause)),
      repeat(seq(optional($._nl), $.foreign_contract)),
    ),
    abstract_state: $ => seq(
      ...declPrefix($), '@', field('name', $._name),
      '{', repeat(choice($.field_declaration, $._terminator, ',', $.attribute)), '}',
    ),

    type_alias_declaration: $ => seq(
      ...declPrefix($), 'type', field('name', $._name),
      optional(field('type_parameters', $.generic_parameters)),
      '=', field('type', $._type),
    ),

    extern_block: $ => seq(
      ...declPrefix($), 'extern', optional(field('abi', choice($.string_literal, $.identifier))),
      '{', repeat(choice($.procedure_signature, $._terminator, $.attribute)), '}',
    ),
    foreign_contract: $ => seq(
      '|:', '@', field('kind', $.identifier), token.immediate('('),
      optional(seq(field('qualifier', $.contract_intrinsic), ':')),
      $._expression, ')',
    ),

    derive_target_declaration: $ => seq(
      'derive', 'target', field('name', $._name),
      '(', 'target', ':', $._type, ')',
      optional(seq('|:', commaSep1($.derive_clause))),
      field('body', $.block),
    ),
    derive_clause: $ => seq(choice('emits', 'requires'), $._name),

    // ---- attributes ------------------------------------------------------

    attribute: $ => seq(
      '#', field('name', $.attribute_name),
      optional(field('arguments', $.attribute_arguments)),
    ),
    attribute_name: $ => seq($._attribute_word, repeat(seq('::', $._attribute_word))),
    _attribute_word: $ => choice($.identifier, 'static', 'dynamic'),
    attribute_arguments: $ => seq(token.immediate('('), commaSep($.attribute_argument), ')'),
    attribute_argument: $ => choice(
      $._literal,
      $._attribute_word,
      seq(field('key', $._attribute_word), ':', choice($._literal, $._attribute_word)),
      seq(field('key', $._attribute_word), '(', commaSep($.attribute_argument), ')'),
    ),

    // ---- types -----------------------------------------------------------

    _type: $ => choice(
      $.union_type,
      $._type_no_union,
    ),
    _type_no_union: $ => choice(
      $.permission_type,
      $.refinement_type,
      $.type_path,
      $.generic_type,
      $.state_type,
      $.unit_type,
      $.never_type,
      $.tuple_type,
      $.array_type,
      $.slice_type,
      $.function_type,
      $.closure_type,
      $.raw_pointer_type,
      $.dynamic_type,
      $.opaque_type,
    ),
    union_type: $ => prec.left(1, seq($._type, '|', $._type)),
    permission_type: $ => prec.right(2, seq(choice('const', 'unique', 'shared'), $._type_no_union)),
    refinement_type: $ => prec.left(3, seq($._type_no_union, '|:', $.predicate_block)),
    type_path: $ => prec.left(seq($._name, repeat(seq('::', $._name)))),
    generic_type: $ => seq($.type_path, $.generic_arguments),
    state_type: $ => seq(
      choice($.type_path, $.generic_type), '@', field('state', $._name),
    ),
    unit_type: _ => seq('(', ')'),
    never_type: _ => '!',
    tuple_type: $ => seq(
      '(', $._type, choice(';', seq(repeat(seq(',', $._type)), optional(','))), ')',
    ),
    array_type: $ => seq('[', field('element', $._type), ';', field('length', $._expression), ']'),
    slice_type: $ => seq('[', field('element', $._type), ']'),
    parameter_types: $ => seq('(', commaSep(seq(optional('move'), $._type)), ')'),
    function_type: $ => prec.right(seq($.parameter_types, '->', $._type)),
    closure_type: $ => prec.right(seq(
      '|', commaSep(seq(optional('move'), $._type_no_union)), '|', '->', $._type,
      optional($.closure_dependencies),
    )),
    closure_dependencies: $ => seq(
      '[', 'shared', ':', '{', commaSep(seq($._name, ':', $._type)), '}', ']',
    ),
    raw_pointer_type: $ => prec.right(2, seq('*', choice('imm', 'mut'), $._type_no_union)),
    dynamic_type: $ => prec.right(seq('$', $.type_path, optional($.generic_arguments))),
    opaque_type: $ => prec.right(seq('opaque', $.type_path, optional($.generic_arguments))),

    // ---- patterns --------------------------------------------------------

    _pattern: $ => choice(
      $.literal_pattern,
      $.wildcard_pattern,
      $.identifier_pattern,
      $.typed_pattern,
      $.tuple_pattern,
      $.record_pattern,
      $.enum_pattern,
      $.modal_pattern,
      $.range_pattern,
      $.splice,
    ),
    literal_pattern: $ => choice($._literal, seq('-', choice($.integer_literal, $.float_literal))),
    wildcard_pattern: _ => '_',
    identifier_pattern: $ => $.identifier,
    typed_pattern: $ => prec.dynamic(-1, prec.right(seq(choice('_', $._name), ':', $._type_no_union))),
    tuple_pattern: $ => seq(
      '(',
      optional(seq($._pattern, choice(';', seq(repeat(seq(',', $._pattern)), optional(','))))),
      ')',
    ),
    record_pattern: $ => seq(field('type', $.type_path), '{', commaSep($.field_pattern), '}'),
    field_pattern: $ => seq(
      field('name', $._name), optional(seq(':', field('pattern', $._pattern))),
    ),
    enum_pattern: $ => seq(
      $._name, repeat1(seq('::', $._name)),
      optional(choice(
        seq('(', commaSep($._pattern), ')'),
        seq('{', commaSep($.field_pattern), '}'),
      )),
    ),
    modal_pattern: $ => seq(
      '@', field('state', $._name), optional(seq('{', commaSep($.field_pattern), '}')),
    ),
    range_pattern: $ => prec.left(PREC.range, seq($._pattern, choice('..', '..='), $._pattern)),

    // ---- statements ------------------------------------------------------

    block: $ => seq(
      '{',
      repeat($._terminator),
      optional(seq(
        $._block_element,
        repeat(seq(repeat1($._terminator), $._block_element)),
        repeat($._terminator),
      )),
      '}',
    ),

    _statement: $ => choice(
      $.let_statement,
      $.using_statement,
      $.assignment_statement,
      $.expression_statement,
      $.return_statement,
      $.break_statement,
      $.continue_statement,
      $.defer_statement,
      $.region_statement,
      $.frame_statement,
      $.key_block_statement,
      $.import_declaration,
      $.using_declaration,
      $.procedure_declaration,
      $.comptime_procedure_declaration,
      $.record_declaration,
      $.enum_declaration,
      $.modal_declaration,
      $.class_declaration,
      $.type_alias_declaration,
      $.extern_block,
      $.derive_target_declaration,
    ),

    _block_element: $ => seq(repeat($.attribute), $._statement),

    let_statement: $ => seq(
      choice('let', 'var'), field('pattern', $._pattern),
      optional(seq(':', field('type', $._type))),
      choice('=', ':='), field('value', $._value),
    ),
    using_statement: $ => seq('using', field('source', $._name), 'as', field('alias', $._name)),
    assignment_statement: $ => seq(
      field('left', $._expression),
      field('operator', choice('=', '+=', '-=', '*=', '/=', '%=', '&=', '|=', '^=', '<<=', '>>=')),
      field('right', $._value),
    ),
    expression_statement: $ => $._expression,
    return_statement: $ => prec.right(seq('return', optional($._value))),
    break_statement: $ => prec.right(seq('break', optional($._expression))),
    continue_statement: _ => 'continue',
    defer_statement: $ => seq('defer', $.block),
    region_statement: $ => seq(
      'region',
      optional(seq('(', field('options', $._expression), ')')),
      optional(seq('as', field('alias', $._name))),
      field('body', $.block),
    ),
    frame_statement: $ => choice(seq('frame', $.block), $.labeled_frame),
    labeled_frame: $ => seq(field('target', $.identifier), '.', 'frame', $.block),

    key_block_statement: $ => seq(
      '%',
      field('kind', choice(
        'read', 'write', seq('release', choice('read', 'write')),
        seq('speculative', choice('read', 'write')),
      )),
      commaSep1($.key_path),
      optional($.option_list),
      field('body', $.block),
    ),
    key_path: $ => prec.right(seq(
      field('root', $.identifier),
      repeat(choice(
        seq('.', optional('#'), $._name),
        seq('[', optional('#'), $._expression, ']'),
      )),
    )),

    option_list: $ => seq(alias($._option_bracket, '['), commaSep1($.option), ']'),
    option: $ => choice(
      seq(field('name', $.identifier), ':', field('value', choice($._expression, '+', '*'))),
      field('name', $.identifier),
    ),

    // ---- expressions -----------------------------------------------------

    _expression: $ => choice(
      $._literal,
      $.identifier,
      $.scoped_identifier,
      $.unit,
      $.parenthesized_expression,
      $.tuple_expression,
      $.array_expression,
      $.record_literal,
      $.modal_literal,
      $.enum_literal,
      $.closure_expression,
      $.if_expression,
      $.loop_expression,
      $.block,
      $.unsafe_block,
      $.unary_expression,
      $.binary_expression,
      $.cast_expression,
      $.range_expression,
      $.pipeline_expression,
      $.field_expression,
      $.index_expression,
      $.call_expression,
      $.generic_call,
      $.method_call_expression,
      $.propagate_expression,
      $.alloc_expression,
      $.transmute_expression,
      $.sizeof_expression,
      $.sync_expression,
      $.wait_expression,
      $.yield_expression,
      $.race_expression,
      $.all_expression,
      $.spawn_expression,
      $.parallel_expression,
      $.dispatch_expression,
      $.comptime_expression,
      $.quote_expression,
      $.type_literal,
      $.splice,
      $.contract_intrinsic,
    ),

    _value: $ => seq(repeat($.attribute), $._expression),

    _literal: $ => choice(
      $.integer_literal, $.float_literal, $.string_literal, $.char_literal, $.boolean_literal,
      $.null_literal,
    ),
    integer_literal: _ => token(seq(
      choice(
        /0x[0-9a-fA-F](_*[0-9a-fA-F])*/,
        /0o[0-7](_*[0-7])*/,
        /0b[01](_*[01])*/,
        DEC,
      ),
      optional('_'),
      INT_SUFFIX,
    )),
    float_literal: _ => token(choice(
      seq(DEC, '.', DEC, optional(seq(/[eE][+-]?/, DEC)), optional('_'), optional(/f(16|32|64)?/)),
      seq(DEC, /[eE][+-]?/, DEC, optional('_'), optional(/f(16|32|64)?/)),
      seq(DEC, optional('_'), /f(16|32|64)/),
    )),
    string_literal: $ => seq(
      '"',
      repeat(choice(alias(token.immediate(prec(2, /[^"\\\n]+/)), $.string_content), $.escape_sequence)),
      '"',
    ),
    char_literal: $ => seq(
      "'",
      choice(alias(token.immediate(prec(2, /[^'\\\n]/)), $.char_content), $.escape_sequence),
      "'",
    ),
    escape_sequence: _ => token.immediate(seq(
      '\\',
      choice(/[^xu\n]/, /x[0-9a-fA-F]{2}/, /u\{[0-9a-fA-F]+\}/),
    )),
    boolean_literal: _ => choice('true', 'false'),
    null_literal: _ => 'null',
    unit: _ => seq('(', ')'),

    scoped_identifier: $ => prec.left(PREC.path, seq(
      field('path', choice($.identifier, $.scoped_identifier, $.generic_name)),
      '::', field('name', $._name),
    )),

    parenthesized_expression: $ => seq('(', $._value, ')'),
    tuple_expression: $ => seq(
      '(', $._expression, choice(';', seq(repeat1(seq(',', $._expression)), optional(','))), ')',
    ),
    array_expression: $ => seq(
      '[', commaSep(choice($._expression, $.array_repeat)), ']',
    ),
    array_repeat: $ => seq(field('value', $._expression), ';', field('count', $._expression)),

    record_literal: $ => seq(
      field('type', choice($.identifier, $.scoped_identifier, $.state_name, $.generic_name)),
      field('body', $.field_initializer_list),
    ),
    modal_literal: $ => seq('@', field('state', $._name), field('body', $.field_initializer_list)),
    enum_literal: $ => seq(
      field('type', choice($.identifier, $.scoped_identifier, $.generic_name)), '::',
      field('variant', $._name), field('body', $.field_initializer_list),
    ),
    field_initializer_list: $ => seq('{', commaSep($.field_initializer), '}'),
    field_initializer: $ => seq(
      field('name', $._name), optional(seq(':', field('value', $._expression))),
    ),

    closure_expression: $ => prec.right(seq(
      choice($.closure_parameters, '||'),
      optional(seq('->', field('return_type', $._type_no_union))),
      field('body', $._expression),
    )),
    closure_parameters: $ => seq('|', commaSep($.closure_parameter), '|'),
    closure_parameter: $ => seq(
      optional('move'), field('name', $._name), optional(seq(':', field('type', $._type_no_union))),
    ),

    if_expression: $ => prec.right(seq(
      'if', field('condition', $._expression),
      choice(
        seq(field('consequence', $.block), optional($.else_clause)),
        seq('is', $._if_case_pattern, field('consequence', $.block), optional($.else_clause)),
        seq('is', $.case_block),
      ),
    )),
    _if_case_pattern: $ => choice($._pattern, $.type_case),
    type_case: $ => seq(':', $._type_no_union),
    else_clause: $ => seq('else', choice($.block, $.if_expression)),
    case_block: $ => seq('{', repeat(choice($.case_arm, $._terminator, ',')), optional($.else_clause), repeat($._terminator), '}'),
    case_arm: $ => seq($._if_case_pattern, $.block),

    loop_expression: $ => seq(
      'loop',
      optional(choice(
        field('condition', $._expression),
        seq(field('pattern', $._pattern), optional(seq(':', field('type', $._type))),
          'in', field('iterator', $._expression)),
      )),
      optional($.loop_invariant),
      field('body', $.block),
    ),
    loop_invariant: $ => seq('|:', $.predicate_block),

    unsafe_block: $ => seq('unsafe', $.block),

    unary_expression: $ => prec(PREC.unary, seq(
      field('operator', choice('!', '-', '&', '*', 'move', 'copy', 'widen', 'new')),
      field('operand', $._expression),
    )),

    binary_expression: $ => {
      const table = [
        [PREC.or, '||'],
        [PREC.and, '&&'],
        [PREC.compare, choice('==', '!=', '<', '<=', '>', '>=')],
        [PREC.bit_or, '|'],
        [PREC.bit_xor, '^'],
        [PREC.bit_and, '&'],
        [PREC.shift, choice('<<', '>>')],
        [PREC.add, choice('+', '-')],
        [PREC.mul, choice('*', '/', '%')],
      ];
      return choice(
        ...table.map(([precedence, operator]) => prec.left(precedence, seq(
          field('left', $._expression), field('operator', operator), field('right', $._expression),
        ))),
        prec.right(PREC.pow, seq(
          field('left', $._expression), field('operator', '**'), field('right', $._expression),
        )),
      );
    },

    cast_expression: $ => prec.left(PREC.cast, seq(
      field('value', $._expression), 'as', field('type', $._type_no_union),
    )),

    range_expression: $ => choice(
      prec.left(PREC.range, seq($._expression, choice('..', '..='), $._expression)),
      prec.right(PREC.range, seq($._expression, '..')),
      prec.right(PREC.range, seq(choice('..', '..='), $._expression)),
      prec.right(PREC.range, '..'),
    ),

    pipeline_expression: $ => prec.left(PREC.pipeline, seq($._expression, '=>', $._expression)),

    field_expression: $ => prec(PREC.postfix, seq(
      field('value', $._expression), '.',
      field('field', choice($._name, $.integer_literal)),
    )),
    index_expression: $ => prec(PREC.postfix, seq(
      field('value', $._expression), '[', field('index', $._expression), ']',
    )),
    call_expression: $ => prec(PREC.postfix, seq(
      field('function', $._expression), field('arguments', $.arguments),
    )),
    generic_call: $ => prec(PREC.postfix, seq(
      field('function', $._expression),
      field('type_arguments', alias($.expression_generic_arguments, $.generic_arguments)),
      field('arguments', $.arguments),
    )),
    expression_generic_arguments: $ => seq(alias($._generic_open, '<'), commaSep($._type), '>'),
    generic_name: $ => seq(
      choice($.identifier, $.scoped_identifier),
      alias($.expression_generic_arguments, $.generic_arguments),
    ),
    state_name: $ => seq(
      choice($.identifier, $.scoped_identifier, $.generic_name), '@', field('state', $._name),
    ),
    method_call_expression: $ => prec(PREC.postfix, seq(
      field('receiver', $._expression), '~>', field('name', $._name),
      field('arguments', $.arguments),
    )),
    arguments: $ => seq('(', commaSep(seq(optional(choice('move', 'copy')), $._expression)), ')'),
    propagate_expression: $ => prec(PREC.postfix, seq($._expression, '?')),

    alloc_expression: $ => prec(PREC.unary, seq(
      optional(field('region', $.identifier)), '^', field('value', $._expression),
    )),
    transmute_expression: $ => seq(
      'transmute', '<', $._type, ',', $._type, '>', '(', $._expression, ')',
    ),
    sizeof_expression: $ => seq(choice('sizeof', 'alignof'), '(', $._type, ')'),

    sync_expression: $ => prec.right(PREC.unary, seq('sync', $._expression)),
    wait_expression: $ => prec.right(PREC.unary, seq('wait', $._expression)),
    yield_expression: $ => prec.right(seq(
      'yield', optional('release'), optional('from'), $._expression,
    )),
    race_expression: $ => seq('race', '{', commaSep($.race_arm), '}'),
    race_arm: $ => seq(
      field('operation', $._expression), '->', '|', field('pattern', $._pattern), '|',
      field('handler', $._expression),
    ),
    all_expression: $ => seq('all', '{', commaSep($._expression), '}'),
    spawn_expression: $ => seq('spawn', optional($.option_list), field('body', $.block)),
    parallel_expression: $ => seq(
      'parallel', field('domain', $._expression), optional($.option_list), field('body', $.block),
    ),
    dispatch_expression: $ => seq(
      'dispatch', field('pattern', $._pattern), 'in', field('range', $._expression),
      optional($.key_clause), optional($.option_list), field('body', $.block),
    ),
    key_clause: $ => seq('key', $.key_path, choice('read', 'write')),

    comptime_expression: $ => seq(
      'comptime',
      choice(
        $.block,
        seq('if', field('condition', $._expression), $.block, optional($.comptime_else)),
        seq('loop', field('pattern', $._pattern), optional(seq(':', $._type)),
          'in', field('iterator', $._expression), $.block),
      ),
    ),
    comptime_else: $ => seq('else', choice($.block, seq('comptime', 'if', $._expression, $.block, optional($.comptime_else)), seq('if', $._expression, $.block, optional($.comptime_else)))),

    quote_expression: $ => seq(
      'quote',
      choice(
        $.block,
        seq('type', '{', $._type, '}'),
        seq('pattern', '{', $._pattern, '}'),
      ),
    ),
    type_literal: $ => seq('Type', '::', '<', $._type, '>'),
    splice: $ => choice(
      seq('$', '(', $._expression, ')'),
      seq('$', $.identifier),
    ),
    contract_intrinsic: $ => prec.right(seq(
      '@', field('name', $.identifier), optional(seq(token.immediate('('), $._expression, ')')),
    )),

    // ---- names -----------------------------------------------------------

    _name: $ => choice($.identifier, $.splice),

    identifier: _ => /[_\p{XID_Start}][\p{XID_Continue}]*/u,
  },
});
