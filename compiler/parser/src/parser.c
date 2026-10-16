#include <stdio.h>
#include <string.h>
#include "../../lexer/include/lexer.h"
#include "../include/grammer.h"

int main(void) {
    char input[256];

    printf("Enter Query: ");
    if (fgets(input, sizeof(input), stdin) == NULL) {
        printf("Invalid Query\n");
        return 1;
    }

    input[strcspn(input, "\n")] = 0;

    Token tokens[MAX_TOKENS] = {0};
    int count = lex(input, tokens);

    QueryAST ast = {0};
    char sql[512];

    if (parse_query(tokens, count, &ast) && query_to_sql(&ast, sql, sizeof(sql))) {
        printf("%s\n", sql);
    } else {
        printf("Invalid Query\n");
        return 1;
    }

    return 0;
}
