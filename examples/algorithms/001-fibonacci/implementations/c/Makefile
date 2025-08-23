CC = gcc
CFLAGS = -O3 -Wall -Wextra -std=c11 -march=native -mtune=native
LDFLAGS = -lm

TARGET = fibonacci
SRC = fibonacci.c

all: $(TARGET)

$(TARGET): $(SRC)
	$(CC) $(CFLAGS) -o $(TARGET) $(SRC) $(LDFLAGS)

test: $(TARGET)
	./$(TARGET) test

benchmark: $(TARGET)
	./$(TARGET) benchmark

clean:
	rm -f $(TARGET) *.o

.PHONY: all test benchmark clean