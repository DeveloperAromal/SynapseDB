#ifndef GRAMMAR_H
#define GRAMMAR_H

#include "../../lexer/include/token.h"

typedef struct {
    char *action;
    char *field;
    char *table;
    char *condition;
} QueryAST;

int parse_select(Token tokens[], int count, QueryAST *ast);
int parse_where(Token tokens[], int count, QueryAST *ast);
int parse_insert(Token tokens[], int count, QueryAST *ast);
int parse_query(Token tokens[], int count, QueryAST *ast);

#endif /* GRAMMAR_H */