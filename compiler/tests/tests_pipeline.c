#include <stdio.h>
#include <string.h>
#include "../lexer/include/lexer.h"
#include "../parser/include/grammer.h"

static int failures = 0;

/* Runs one sentence through the lexer and parser. expected is the SQL it
   should produce, or NULL if parsing should fail. */
static void check(const char *sentence, const char *expected) {
    char input[1024];
    snprintf(input, sizeof(input), "%s", sentence);

    Token tokens[MAX_TOKENS] = {0};
    int count = lex(input, tokens);

    QueryAST ast = {0};
    char sql[512] = "";
    int ok = parse_query(tokens, count, &ast) && query_to_sql(&ast, sql, sizeof(sql));

    if (expected == NULL ? !ok : ok && strcmp(sql, expected) == 0) {
        printf("PASS  %s\n", sentence);
        return;
    }

    failures++;
    printf("FAIL  %s\n      expected: %s\n      got:      %s\n", sentence,
           expected ? expected : "(parse failure)", ok ? sql : "(parse failure)");
    for (int i = 0; i < count; i++)
        printf("      %2d | %-10s | %s\n", i, token_type_name(tokens[i].type), tokens[i].value);
}

int main(void) {
    check("show name from users where age > 30",
          "SELECT name FROM users WHERE age > 30;");
    check("select name from users",
          "SELECT name FROM users;");
    check("Please show me the email of customers",
          "SELECT email FROM customers;");
    check("list email of customers where age greater than 30",
          "SELECT email FROM customers WHERE age > 30;");
    check("get name from users with age at least 18",
          "SELECT name FROM users WHERE age >= 18;");
    check("find name from users where city is not null",
          "SELECT name FROM users WHERE city IS NOT null;");
    check("show name from users where age equal to 30",
          "SELECT name FROM users WHERE age = 30;");
    check("show\tname\r\nfrom users",
          "SELECT name FROM users;");

    /* Malformed input must fail cleanly. */
    check("name from users", NULL);
    check("show name from", NULL);
    check("", NULL);

    /* A word longer than a token's buffer is cut short, not overflowed. */
    char long_word[200];
    memset(long_word, 'a', sizeof(long_word) - 1);
    long_word[sizeof(long_word) - 1] = '\0';
    char sentence[300];
    snprintf(sentence, sizeof(sentence), "show %s from users", long_word);
    char expected[300];
    snprintf(expected, sizeof(expected), "SELECT %.*s FROM users;",
             (int)sizeof(((Token *)0)->value) - 1, long_word);
    check(sentence, expected);

    /* More words than MAX_TOKENS must not overflow the token array. */
    char many_words[MAX_TOKENS * 2 * 2 + 64] = "show name from users where";
    for (int i = 0; i < MAX_TOKENS * 2; i++)
        strcat(many_words, " x");
    char input[sizeof(many_words)];
    memcpy(input, many_words, sizeof(many_words));
    Token tokens[MAX_TOKENS];
    int count = lex(input, tokens);
    if (count != MAX_TOKENS || tokens[count - 1].type != TOKEN_EOF) {
        failures++;
        printf("FAIL  token limit: got %d tokens, want %d ending in EOF\n", count, MAX_TOKENS);
    } else {
        printf("PASS  token limit\n");
    }

    printf("\n%s: %d failure(s)\n", failures ? "FAILED" : "OK", failures);
    return failures ? 1 : 0;
}
