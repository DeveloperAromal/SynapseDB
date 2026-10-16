#ifndef LEXER_H
#define LEXER_H

#include "token.h"

/* Number of entries the tokens array passed to lex() must have room for,
   including the final TOKEN_EOF. Words beyond that are ignored. */
#define MAX_TOKENS 256

/* Lower-cases input in place and splits it into tokens. Returns the number
   of tokens written, TOKEN_EOF included. */
int lex(char *input, Token tokens[]);

#endif /* LEXER_H */
