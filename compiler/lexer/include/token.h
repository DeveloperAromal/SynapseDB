#ifndef TOKEN_H
#define TOKEN_H

typedef enum {
    TOKEN_KEYWORD,
    TOKEN_IDENTIFIER,
    TOKEN_OPERATOR,
    TOKEN_NUMBER,
    TOKEN_STRING,
    TOKEN_EOF,
} TokenType;

typedef struct {
    TokenType type;
    char value[64];
} Token;

const char *token_type_name(TokenType type);

#endif /* TOKEN_H */
