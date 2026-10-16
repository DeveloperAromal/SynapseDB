#ifndef VOCAB_H
#define VOCAB_H

typedef struct {
    const char *word;
    const char *mapped;
} Mapping;

/* Each table ends with a NULL entry. The tables are defined in vocab.c so
   that every file including this header shares one copy. */

extern const Mapping KEYWORDS[];

/* Two-word operators such as "greater than": {{first, NULL}, {second, op}}. */
extern const Mapping MULTI_OPS[][2];

extern const Mapping OPERATORS[];

extern const char *const STOP_WORDS[];

#endif /* VOCAB_H */
