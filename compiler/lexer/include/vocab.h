#ifndef VOCAB_H
#define VOCAB_H



#include <stddef.h>



typedef struct {
    const char *word;
    const char *mapped;
} Mapping;




static Mapping KEYWORDS[] = {
    /* SELECT */
    {"select", "SELECT"},
    {"show", "SELECT"},
    {"get", "SELECT"},
    {"list", "SELECT"},
    {"display", "SELECT"},
    {"fetch", "SELECT"},
    {"retrieve", "SELECT"},
    {"find", "SELECT"},
    {"view", "SELECT"},

    /* FROM */
    {"from", "FROM"},
    {"in", "FROM"},
    {"inside", "FROM"},
    {"within", "FROM"},
    {"of", "FROM"},

    /* WHERE */
    {"where", "WHERE"},
    {"when", "WHERE"},
    {"if", "WHERE"},
    {"with", "WHERE"},
    {"having", "HAVING"},
    {"filter", "WHERE"},
    {"filtered", "WHERE"},
    {"condition", "WHERE"},

    /* LOGIC */
    {"and", "AND"},
    {"or", "OR"},
    {"not", "NOT"},

    /* ORDER */
    {"order", "ORDER"},
    {"sort", "ORDER"},
    {"sorted", "ORDER"},
    {"arrange", "ORDER"},
    {"by", "BY"},
    {"ascending", "ASC"},
    {"descending", "DESC"},
    {"asc", "ASC"},
    {"desc", "DESC"},

    /* GROUP */
    {"group", "GROUP"},
    {"grouped", "GROUP"},
    {"aggregate", "GROUP"},
    {"aggregated", "GROUP"},

    /* LIMIT / OFFSET */
    {"limit", "LIMIT"},
    {"top", "LIMIT"},
    {"first", "LIMIT"},
    {"last", "LIMIT"},
    {"offset", "OFFSET"},
    {"skip", "OFFSET"},

    /* INSERT */
    {"insert", "INSERT"},
    {"add", "INSERT"},
    {"create", "INSERT"},
    {"new", "INSERT"},
    {"into", "INTO"},
    {"values", "VALUES"},

    /* UPDATE */
    {"update", "UPDATE"},
    {"modify", "UPDATE"},
    {"change", "UPDATE"},
    {"set", "SET"},
    {"replace", "UPDATE"},

    /* DELETE */
    {"delete", "DELETE"},
    {"remove", "DELETE"},
    {"drop", "DELETE"},
    {"erase", "DELETE"},

    /* JOINS */
    {"join", "JOIN"},
    {"inner", "INNER"},
    {"left", "LEFT"},
    {"right", "RIGHT"},
    {"full", "FULL"},
    {"outer", "OUTER"},
    {"cross", "CROSS"},
    {"on", "ON"},

    /* FUNCTIONS */
    {"count", "COUNT"},
    {"sum", "SUM"},
    {"avg", "AVG"},
    {"average", "AVG"},
    {"min", "MIN"},
    {"max", "MAX"},

    /* DISTINCT */
    {"distinct", "DISTINCT"},
    {"unique", "DISTINCT"},

    /* EXISTS */
    {"exists", "EXISTS"},
    {"between", "BETWEEN"},
    {"like", "LIKE"},
    {"in", "IN"},

    {NULL, NULL}};


    


    
static Mapping MULTI_OPS[][2] = {{{"greater", NULL}, {"than", ">"}},
                                 {{"less", NULL}, {"than", "<"}},
                                 {{"greater", NULL}, {"equal", ">="}},
                                 {{"less", NULL}, {"equal", "<="}},
                                 {{"not", NULL}, {"equal", "!="}},
                                 {{"equal", NULL}, {"to", "="}},
                                 {{"different", NULL}, {"from", "!="}},
                                 {{"at", NULL}, {"least", ">="}},
                                 {{"at", NULL}, {"most", "<="}},
                                 {{"more", NULL}, {"than", ">"}},
                                 {{"fewer", NULL}, {"than", "<"}},
                                 {{"no", NULL}, {"more", "<="}},
                                 {{"no", NULL}, {"less", ">="}},
                                 {{"starts", NULL}, {"with", "LIKE"}},
                                 {{"ends", NULL}, {"with", "LIKE"}},
                                 {{"contains", NULL}, {"value", "LIKE"}},
                                 {{"is", NULL}, {"null", "IS NULL"}},
                                 {{"is", NULL}, {"not", "IS NOT"}},
                                 {{"not", NULL}, {"in", "NOT IN"}},
                                 {{"greater", NULL}, {"than=", ">="}},
                                 {{"less", NULL}, {"than=", "<="}}};




static Mapping OPERATORS[] = {{"=", "="},
                              {">", ">"},
                              {"<", "<"},
                              {">=", ">="},
                              {"<=", "<="},
                              {"!=", "!="},
                              {"<>", "!="},
                              {"is", "="},
                              {"equals", "="},
                              {"like", "LIKE"},
                              {"between", "BETWEEN"},
                              {"in", "IN"},
                              {"not", "NOT"},
                              {"exists", "EXISTS"},
                              {"+", "+"},
                              {"-", "-"},
                              {"*", "*"},
                              {"/", "/"},
                              {"%", "%"},
                              {"&&", "AND"},
                              {"||", "OR"},
                              {"!", "NOT"},
                              {NULL, NULL}};




static const char *STOP_WORDS[] = {
    "the",    "a",      "an",      "me",     "my",    "mine",   "you",
    "your",   "yours",  "we",      "our",    "ours",  "they",   "them",
    "their",  "theirs", "please",  "kindly", "just",  "simply", "basically",
    "who",    "whom",   "which",   "that",   "those", "these",  "is",
    "are",    "was",    "were",    "be",     "been",  "being",  "do",
    "does",   "did",    "done",    "can",    "could", "should", "would",
    "may",    "might",  "will",    "shall",  "all",   "any",    "some",
    "many",   "few",    "several", "each",   "every", "either", "neither",
    "here",   "there",  "when",    "where",  "why",   "how",    "please",
    "thanks", "thank",  "ok",      "okay",   "tell",  "show",   "give",
    "want",   "need",   "about",   "for",    "to",    "with",   "without",
    "as",     "at",     "by",      "on",     "off",   "up",     "down",
    NULL};

#endif /* VOCAB_H */
