#include "helpers.h"
#include <ctype.h>
#include <string.h>


void to_lower_case(char *s) {

    for (; *s; ++s) {

        *s = (char)tolower((unsigned char)*s);

        putchar(*s);
    }

    putchar('\n');
}



int is_stop_word(const char *word) {

    for (int i = 0; STOP_WORDS[i] != NULL; i++) {
        
        if (strcmp(STOP_WORDS[i], word) == 0) 
            return 1;

    }

    return 0;

}



const char *lookup(Mapping *table, const char *word) {

    for (int i =  0; table[i].words != NULL; i++) {

        if (strcmp(table[i].words, word) == 0) {

            return table[i].mapped;

        }

    }

    return NULL

}



int is_number(const  char *s) {

    if (*s == '\0') 

        return 0;

    while (*s) {

        if (!isdigit((unsigned char)*s))

            return 0;

        s++;

    }

    return 1;

}