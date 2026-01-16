#ifndef VOCAB_H
#define VOCAB_H



typedef struct {
    const char *words;
    const char *mapped;
} Mapping;


static Mapping KEYWORDS[] = {
    {"show", "SELECT"},
    {"get", "SELECT"},
    {"list", "SELECT"},
    {"where", "WHERE"},
    {"and", "AND"},
    {"or", "OR"},
    {"order", "ORDER"},
    {"by", "BY"},
    {"limit", "LIMIT"},
    {NULL, NULL}
}


static Mapping MULTI_OPS[][2] = {
    {{"greater", NULL}, {"than", ">"}},
    {{"less", NULL}, {"than", "<"}},
    {{"not", NULL}, {"equal", "!="}}
};



static Mapping OPERATORS[] = {
    {"is", "="},
    {"equals", "="},
    {">", ">"},
    {"<", "<"},
    {"=", "="},
    {NULL, NULL}
};


static const char *STOP_WORDS[] = {
    "the", "a", "an", "me", "all", "please", "who", "that", "are", NULL
};

#endif /* VOCAB_H */
