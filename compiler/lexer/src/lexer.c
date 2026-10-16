#include "lexer.h"
#include "helpers.h"

#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>



int word_splitter(char *s, char *words[], int max_tokens) {

    int count = 0;
    char *token = strtok(s, " \t\r\n");

    while (token != NULL && count < max_tokens) {

        words[count++] = token;
        token = strtok(NULL, " \t\r\n");
    }

    return count;
}

/* Copies value into the token, cutting it short instead of overflowing the
   fixed-size buffer when a word is too long. */
static void set_token(Token *token, TokenType type, const char *value) {

    token->type = type;
    snprintf(token->value, sizeof(token->value), "%s", value);
}

static const char *lookup_multi_op(const char *first, const char *second) {

    for (int i = 0; MULTI_OPS[i][0].word != NULL; i++) {

        if (strcmp(MULTI_OPS[i][0].word, first) == 0 &&
            strcmp(MULTI_OPS[i][1].word, second) == 0) {

            return MULTI_OPS[i][1].mapped;
        }
    }

    return NULL;
}

int tokenizer(char *words[], int word_count, Token tokens[]) {

    int token_count = 0;

    /* Leave room for the EOF token. */
    for (int i = 0; i < word_count && token_count < MAX_TOKENS - 1; i++) {

        char *w = words[i];

        /* Checked before stop words: the second word of an operator such
           as "equal to" or "at least" may be a stop word on its own. */
        if (i + 1 < word_count) {

            const char *op = lookup_multi_op(w, words[i + 1]);

            if (op) {

                set_token(&tokens[token_count++], TOKEN_OPERATOR, op);
                i++;
                continue;
            }
        }

        if (is_stop_word(w))

            continue;

        const char *kw = lookup(KEYWORDS, w);

        if (kw) {

            set_token(&tokens[token_count++], TOKEN_KEYWORD, kw);
            continue;

        }

        const char *op = lookup(OPERATORS, w);

        if (op) {

            set_token(&tokens[token_count++], TOKEN_OPERATOR, op);
            continue;
        }

        if (is_number(w)) {

            set_token(&tokens[token_count++], TOKEN_NUMBER, w);
            continue;
        }

        set_token(&tokens[token_count++], TOKEN_IDENTIFIER, w);

    }


    set_token(&tokens[token_count++], TOKEN_EOF, "");

    return token_count;
}


int lex(char *input, Token tokens[]) {

    char *words[MAX_TOKENS];

    to_lower_case(input);

    int word_count = word_splitter(input, words, MAX_TOKENS);

    return tokenizer(words, word_count, tokens);

}


const char *token_type_name(TokenType type) {
    switch (type) {
        case TOKEN_KEYWORD:   return "KEYWORD";
        case TOKEN_IDENTIFIER:return "IDENTIFIER";
        case TOKEN_OPERATOR:  return "OPERATOR";
        case TOKEN_NUMBER:    return "NUMBER";
        case TOKEN_STRING:    return "STRING";
        case TOKEN_EOF:       return "EOF";
        default:              return "UNKNOWN";
    }
}
