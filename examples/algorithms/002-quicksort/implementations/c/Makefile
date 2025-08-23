CC = gcc
CFLAGS = -Wall -Wextra -O2 -std=c11
LDFLAGS = -lm

TARGET = quicksort
SOURCE = quicksort.c

.PHONY: all clean test benchmark

all: $(TARGET)

$(TARGET): $(SOURCE)
	$(CC) $(CFLAGS) -o $@ $< $(LDFLAGS)

test: $(TARGET)
	./$(TARGET)

benchmark: $(TARGET)
	./$(TARGET)

clean:
	rm -f $(TARGET)

debug: CFLAGS += -g -DDEBUG
debug: $(TARGET)

profile: CFLAGS += -pg
profile: LDFLAGS += -pg
profile: $(TARGET)