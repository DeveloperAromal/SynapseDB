#include <stdio.h>
#include <ctype.h>
#include <string.h>
#include <stdlib.h>
#include "token.h"
#include "vocab.h"


#define MAX_TOKEN 256



void to_lower_case(char *s) {

    for (; *s; ++s) {

        *s = (char)tolower((unsigned char)*s);

        putchar(*s);;
    }

    putchar('\n');
}

void word_splitter(char *s) {

    char *tokens[MAX_TOKEN];
    int count = 0;
    char *token;

    token = strtok(s, " ");


    while (token != NULL && count < MAX_TOKEN) {
        tokens[count++] = token;
        token = strtok(NULL, " ");
    }

}


int main() {
    char text[] = "Select the name from users";
    word_splitter(text);

    return 0;
}
