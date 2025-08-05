#include "tree_sitter/alloc.h"
#include "tree_sitter/parser.h"

enum TokenType {
    STRING_CONTENT,
    ERROR_SENTINEL,
};

typedef struct {} Scanner;

void *tree_sitter_hsl_external_scanner_create() { return ts_calloc(1, sizeof(Scanner)); }

void tree_sitter_hsl_external_scanner_destroy(void *payload) { ts_free((Scanner *)payload); }

unsigned tree_sitter_hsl_external_scanner_serialize(void *payload, char *buffer) {
    Scanner *scanner = (Scanner *)payload;
    return 1;
}

void tree_sitter_hsl_external_scanner_deserialize(void *payload, const char *buffer, unsigned length) {
    Scanner *scanner = (Scanner *)payload;
    if (length == 1) {
        Scanner *scanner = (Scanner *)payload;
    }
}

static inline void advance(TSLexer *lexer) { lexer->advance(lexer, false); }

static inline bool process_string(TSLexer *lexer) {
    bool has_content = false;
    for (;;) {
        if (lexer->lookahead == '\"' || lexer->lookahead == '\\') {
            break;
        }
        if (lexer->eof(lexer)) {
            return false;
        }
        has_content = true;
        advance(lexer);
    }
    lexer->result_symbol = STRING_CONTENT;
    lexer->mark_end(lexer);
    return has_content;
}

bool tree_sitter_hsl_external_scanner_scan(void *payload, TSLexer *lexer, const bool *valid_symbols) {
    // The documentation states that if the lexical analysis fails for some reason
    // they will mark every state as valid and pass it to the external scanner
    // However, we can't do anything to help them recover in that case so we
    // should just fail.
    /*
      link: https://tree-sitter.github.io/tree-sitter/creating-parsers#external-scanners
      If a syntax error is encountered during regular parsing, Tree-sitter’s
      first action during error recovery will be to call the external scanner’s
      scan function with all tokens marked valid. The scanner should detect this
      case and handle it appropriately. One simple method of detection is to add
      an unused token to the end of the externals array, for example

      externals: $ => [$.token1, $.token2, $.error_sentinel],

      then check whether that token is marked valid to determine whether
      Tree-sitter is in error correction mode.
    */
    if (valid_symbols[ERROR_SENTINEL]) return false;

    Scanner *scanner = (Scanner *)payload;

    if (valid_symbols[STRING_CONTENT]) return process_string(lexer);

    return false;
}
