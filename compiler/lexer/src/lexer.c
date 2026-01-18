#include "lexer.h"
#include "helpers.h"

#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_TOKEN 256



int word_splitter(char *s, char *words[], int max_tokens) {

    int count = 0;
    char *token = strtok(s, " ");

    while (token != NULL && count < max_tokens) {

        words[count++] = token;
        token = strtok(NULL, " ");
    }

    return count;
}

int tokenizer(char *words[], int word_count, Token tokens[]) {

    int token_count = 0;

    for (int i = 0; i < word_count; i++)  {

        char *w = words[i];

        if (is_stop_word(w)) 
    
            continue;

        const char *kw = lookup(KEYWORDS, w);

        if (kw) {

            tokens[token_count].type = TOKEN_KEYWORD;
            strcpy(tokens[token_count].value, kw);
            token_count++;
            continue;

        }

        if (is_number(w)) {

            tokens[token_count].type = TOKEN_NUMBER;
            strcpy(tokens[token_count].value, w);
            token_count++;
            continue;
        }

        tokens[token_count].type = TOKEN_IDENTIFIER;
        strcpy(tokens[token_count].value, w);
        token_count++;

    }


    tokens[token_count].type = TOKEN_EOF;
    tokens[token_count].value[0] = '\0';
    token_count++;
 
    return token_count;
}


int lex(char *input, Token tokens[]) {

    char *words[MAX_TOKEN];

    to_lower_case(input);

    int word_count = word_splitter(input, words, MAX_TOKEN);

    return tokenizer(words, word_count, tokens);

}


int main() {
    char text[] = "Select not the name from users";
    Token tokens[MAX_TOKEN];

    int count = lex(text, tokens);

    for (int i = 0; i < count; i++) {
        printf("type=%d value=%s\n",
               tokens[i].type,
               tokens[i].value);
    }

    return 0;
}
