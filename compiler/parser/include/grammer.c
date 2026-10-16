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

    if (tokens[3].type != TOKEN_IDENTIFIER)
        return 0;

    ast->table = tokens[3].value;
    return 1;
}


int parse_where(Token tokens[], int count, QueryAST *ast) {
    for (int i = 0; i < count; i++) {
        if (tokens[i].type == TOKEN_KEYWORD && strcmp(tokens[i].value, "WHERE") == 0) {
            size_t len = 0;
            ast->condition[0] = '\0';

            for (int j = i + 1; j < count && tokens[j].type != TOKEN_EOF; j++) {
                size_t room = sizeof(ast->condition) - len;
                int written = snprintf(ast->condition + len, room, "%s%s",
                                       len > 0 ? " " : "", tokens[j].value);
                if (written < 0 || (size_t)written >= room) {
                    ast->condition[0] = '\0';
                    return 0;
                }
                len += (size_t)written;
            }
            return len > 0;
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


int query_to_sql(const QueryAST *ast, char *out, size_t size) {
    const char *action = ast->action ? ast->action : "";
    const char *field = ast->field ? ast->field : "*";
    const char *table = ast->table ? ast->table : "";
    int written;

    if (ast->condition[0] != '\0')
        written = snprintf(out, size, "%s %s FROM %s WHERE %s;", action, field, table, ast->condition);
    else
        written = snprintf(out, size, "%s %s FROM %s;", action, field, table);

    return written >= 0 && (size_t)written < size;
}
