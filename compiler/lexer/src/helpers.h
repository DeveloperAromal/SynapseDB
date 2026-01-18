#ifndef HELPERS_h
#define HELPERS_h


#include "token.h"
#include "vocab.h"


void to_lower_case(char *s);

int is_stop_word(const char *word);
const char *lookup(Mapping *table, const char *word);

int is_number(const char *s);


#endif /* HELPERS_H */
