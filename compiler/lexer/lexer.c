#include "token.h"
// #include "vocab.h"
#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_TOKEN 256

void to_lower_case(char *s) {

    for (; *s; ++s) {

        *s = (char)tolower((unsigned char)*s);

        putchar(*s);
    }

    putchar('\n');
}

int word_splitter(char *s, char *words[], int max_tokens) {
    
    int count = 0;
    char *token = strtok(s, " ");

    while (token != NULL && count < max_tokens) {

        words[count++] = token;
        token = strtok(NULL, " ");

    }

    return count;
}

int main() {
    char text[] = "Select the name from users";
    char *words[MAX_TOKEN];

    to_lower_case(text);

    int word_count = word_splitter(text, words, MAX_TOKEN);

    printf("Tokens (%d):\n", word_count);

    for (int i = 0; i < word_count; i++) {

        printf("%s\n", words[i]);
        
    }
    
    return 0;
}
