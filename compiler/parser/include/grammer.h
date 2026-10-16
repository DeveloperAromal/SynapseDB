#ifndef GRAMMAR_H
#define GRAMMAR_H

#include <stddef.h>

#include "../../lexer/include/token.h"

typedef struct {
    char *action;
    char *field;
    char *table;
    /* Every token after WHERE, separated by spaces, e.g. "age > 30". */
    char condition[256];
} QueryAST;

int parse_select(Token tokens[], int count, QueryAST *ast);
int parse_where(Token tokens[], int count, QueryAST *ast);
int parse_insert(Token tokens[], int count, QueryAST *ast);
int parse_query(Token tokens[], int count, QueryAST *ast);

/* Writes the query as SQL into out. Returns 0 if it does not fit. */
int query_to_sql(const QueryAST *ast, char *out, size_t size);

#endif /* GRAMMAR_H */
