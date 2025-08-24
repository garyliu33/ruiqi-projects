#include "prize.h"
#include <stdio.h>
#include <stdlib.h>

void initPrizePool(Prize *prize) {
    prize->cards = malloc(sizeof(Card) * NUM_CARDS);
    if (prize->cards == NULL) {
        printf("out of memory\n");
        exit(1);
    }
    prize->size = 0;
}

void addPrizeCard(Prize *prize, Card card) {
    if (prize->size < NUM_CARDS) {
        prize->cards[prize->size] = card;
        prize->size++;
    }
}

void clearPrizes(Prize *prize) {
    prize->size = 0;
}

void printPrizes(Prize *prize) {
    printf("Prize: ");
    for (int i = 0; i < prize->size; i++) {
        printCard(&prize->cards[i]);
        printf(" ");
    }
    printf("\n");
}

void freePrizePool(Prize *prize) {
    if (prize->cards != NULL) {
        free(prize->cards);
        prize->cards = NULL;
        prize->size = 0;
    }
}

int totalPrizeValue(Prize *prize) {
    int value = 0;
    for (int i = 0; i < prize->size; i++) {
        value += prize->cards[i].rank;
    }
    return value;
}