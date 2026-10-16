CFLAGS ?= -std=c11 -O2 -Wall -Wextra
CPPFLAGS += -Ilexer/include -Ilexer/src -Iparser/include

COMMON_SRC = lexer/src/lexer.c lexer/src/helpers.c lexer/src/vocab.c parser/include/grammer.c
HEADERS = $(wildcard lexer/include/*.h lexer/src/*.h parser/include/*.h)
OUT = output

all: $(OUT)/parser

$(OUT)/parser: parser/src/parser.c $(COMMON_SRC) $(HEADERS) | $(OUT)
	$(CC) $(CPPFLAGS) $(CFLAGS) -o $@ parser/src/parser.c $(COMMON_SRC)

$(OUT)/tests_pipeline: tests/tests_pipeline.c $(COMMON_SRC) $(HEADERS) | $(OUT)
	$(CC) $(CPPFLAGS) $(CFLAGS) -o $@ tests/tests_pipeline.c $(COMMON_SRC)

test: $(OUT)/tests_pipeline
	./$(OUT)/tests_pipeline

$(OUT):
	mkdir -p $(OUT)

clean:
	rm -rf $(OUT)

.PHONY: all test clean
