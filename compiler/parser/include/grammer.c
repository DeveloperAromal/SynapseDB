#include "grammer.h"

#include <stdio.h>
#include <string.h>

int parse_select(Token tokens[], int count, QueryAST *ast) {
    if (count < 4)
        return 0;

    if (tokens[0].type != TOKEN_KEYWORD || strcmp(tokens[0].value, "SELECT") != 0)
        return 0;  

    ast->action = "SELECT";
    ast->field = tokens[1].value;

    if (tokens[2].type != TOKEN_KEYWORD || strcmp(tokens[2].value, "FROM") != 0)
        return 0;  

    ast->table = tokens[3].value;
    return 1;
}


int parse_where(Token tokens[], int count, QueryAST *ast) {
    for (int i = 0; i < count; i++) {
        if (tokens[i].type == TOKEN_KEYWORD && strcmp(tokens[i].value, "WHERE") == 0) {
            ast->condition = tokens[i + 1].value;  
            return 1;
        }
    }
    return 0;
}


int parse_insert(Token tokens[], int count, QueryAST *ast) {
    if (count < 4 || strcmp(tokens[0].value, "INSERT") != 0) return 0;
    
    ast->action = "INSERT";
    ast->table = tokens[2].value;
    return 1;
}


int parse_query(Token tokens[], int count, QueryAST *ast) {
    if (!parse_select(tokens, count, ast)) {
        printf("Syntax Error: Invalid SELECT\n");
        return 0;
    }

    parse_where(tokens, count, ast);
    return 1;
}
