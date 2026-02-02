#include <stdio.h>
#include "../lexer/include/lexer.h"
#include "../parser/include/grammer.h"

int main() {
    char input[256] = "show name from users where age > 30";  // ← FIXED: [256]

    Token tokens[256] = {0};
    int count = lex(input, tokens);

    printf("Tokens:\n");
    for (int i = 0; i < count; i++) {
        printf("%2d | %-12s | %s\n",
               i,
               token_type_name(tokens[i].type), 
               tokens[i].value);
    }

    QueryAST ast = {0};
    if (parse_query(tokens, count, &ast)) {
        printf("\nParse OK!\n");
        printf("action    : %s\n", ast.action    ? ast.action    : "(null)");
        printf("field     : %s\n", ast.field     ? ast.field     : "(null)");
        printf("table     : %s\n", ast.table     ? ast.table     : "(null)");
        printf("condition : %s\n", ast.condition ? ast.condition : "(none)");
    } else {
        printf("Parse failed\n");
    }

    return 0;
}
