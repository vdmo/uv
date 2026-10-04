; Neovim (nvim-treesitter) indentation.
[
  (block)
  (record_body)
  (enum_body)
  (modal_body)
  (state_block)
  (class_body)
  (extern_block)
  (case_block)
  (field_initializer_list)
  (using_list)
  (array_expression)
  (arguments)
  (parameters)
  (tuple_expression)
  (race_expression)
  (all_expression)
  (variant_record_payload)
  (variant_tuple_payload)
] @indent.begin

[ "}" ")" "]" ] @indent.branch @indent.end

[ (line_comment) (doc_comment) (block_comment) ] @indent.auto
