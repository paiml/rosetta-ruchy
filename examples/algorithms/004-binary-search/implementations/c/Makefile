CC = gcc
CFLAGS = -std=c11 -Wall -Wextra -Werror -O3 -march=native
TARGET = binary_search
SRC = binary_search.c

.PHONY: all test clean

all: $(TARGET)

$(TARGET): $(SRC)
	$(CC) $(CFLAGS) -o $(TARGET) $(SRC)

test: $(TARGET)
	./$(TARGET) test

clean:
	rm -f $(TARGET) *.o
