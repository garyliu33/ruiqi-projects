# Makefile for building main.exe using MinGW (Windows)

CC = gcc
CFLAGS = -Wall -Wextra -g
SRC = src/main.c src/card.c src/deck.c src/player.c src/prize.c
OUT = main.exe

all: $(OUT)

$(OUT): $(SRC)
	$(CC) $(CFLAGS) -o $(OUT) $(SRC)

clean:
	del /Q *.exe 2>nul
