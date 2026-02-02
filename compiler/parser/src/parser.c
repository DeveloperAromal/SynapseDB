#include <stdio.h>
#include <string.h>
#include "../../lexer/include/lexer.h"
#include "../include/grammer.h"

int main() {
    char input[256];

    printf("Enter Query: ");
    fgets(input, sizeof(input), stdin);

    input[strcspn(input, "\n")] = 0;

    Token tokens[256] = {0};
    int count = lex(input, tokens);

    QueryAST ast = {0};

    if (parse_query(tokens, count, &ast)) {
        if (ast.condition && strlen(ast.condition) > 0) {
            printf("%s %s FROM %s WHERE %s;\n",
                   ast.action ? ast.action : "",
                   ast.field ? ast.field : "*",
                   ast.table ? ast.table : "",
                   ast.condition);
        } else {
            printf("%s %s FROM %s;\n",
                   ast.action ? ast.action : "",
                   ast.field ? ast.field : "*",
                   ast.table ? ast.table : "");
        }
    } else {
        printf("Invalid Query\n");
    }

    return 0;
}
