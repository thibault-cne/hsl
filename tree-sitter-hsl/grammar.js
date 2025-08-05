/**
 * @file Hsl grammar for tree-sitter
 * @author Thibault Cheneviere <thibault.chene23@gmail.com>
 * @license MIT
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

module.exports = grammar({
  name: "hsl",

  externals: $ => [$.string_content],

  extras: $ => [
    /\s/,
    $.line_comment
  ],

  rules: {
    source_file: $ => repeat($._definition),

    _definition: $ => choice(
      $.function_definition,
      $.extern_definition
    ),

    line_comment: $ => seq('<(-.-)>', /.*/),

    extern_definition: $ => seq(
      "Hypersignal",
      repeat(
        seq(
          field("name", $.identifier),
          optional($.variadic)
        )
      ),
      "Jamsignal"
    ),

    function_definition: $ => seq(
      "A long time ago in a",
      field("name", $.identifier),
      "far, far away...",
      optional($.variadic),
      repeat($.statement),
      "May the force be with you."
    ),

    statement: $ => choice(
      $.function_call,
      $.declaration,
      // $.operation,
    ),

    declaration: $ => seq(
      choice(
        "I am a big deal in the resistance.",
        "The force is strong with this one.",
        "That's one hell of a pilot.",
      ),
      $.identifier,
      choice(
        "I am your father.",
        "Judge me by my size, do you ?"
      ),
      $.expression
    ),

    function_call: $ => seq(
      "Execute order",
      field("name", $.identifier),
      repeat(
        $.expression
      ),
      "Order executed"
    ),

    variadic: $ => seq(
      "Starfield",
      field("variadic", $.unsigned_integer)
    ),

    expression: $ => choice($.identifier, $.literal),
    
    identifier: _ => /[a-zA-Z_][a-zA-Z0-9_]*/,

    literal: $ => choice(
      $.integer,
      $.string,
      $.char,
      $.bool
    ),

    string: $ => seq(
      '"',
      repeat(
        choice(
          $.escape_sequence,
          $.string_content
        )
      ),
      token.immediate('"')
    ),

    escape_sequence: _ => token.immediate(
      seq('\\',
        choice(
          /[^xu]/,
          /u[0-9a-fA-F]{4}/,
          /u\{[0-9a-fA-F]+\}/,
          /x[0-9a-fA-F]{2}/,
        ),
      )),

    char: $ => seq(
      "'",
      /[a-zA-Z]/,
      "'"
    ),

    bool: $ => choice(
      "From a certain point of view.",
      "That's impossible!"
    ),

    integer: $ => seq(
      optional("-"),
      $.unsigned_integer
    ),

    unsigned_integer: _ => token(seq(
      choice(
        /[0-9][0-9_]*/,
        /0x[0-9a-fA-F_]+/,
        /0b[01_]+/,
        /0o[0-7_]+/,
      )
    )),
  }
});
