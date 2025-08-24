#include "card.h"
#include <stdio.h>
#include <string.h>

Card createCard(int rank) {
    Card card;
    card.rank = rank;
    return card;
}

int compareCards(Card *a, Card *b) {
    return a->rank - b->rank;
}

void printCard(Card *card) {
    if (card->rank == 1) {
        printf("A");
    } else if (card->rank == 11) {
        printf("J");
    } else if (card->rank == 12) {
        printf("Q");
    } else if (card->rank == 13) {
        printf("K");
    } else {
        printf("%d", card->rank);
    }
}

Card stringToCard(char *s) {
    Card card;
    if (strcmp(s, "A") == 0) {
        card = createCard(1);
    } else if (strcmp(s, "10") == 0) {
        card = createCard(10);
    } else if (strcmp(s, "J") == 0) {
        card = createCard(11);
    } else if (strcmp(s, "Q") == 0) {
        card = createCard(12);
    } else if (strcmp(s, "K") == 0) {
        card = createCard(13);
    } else {
        card = createCard(s[0] - '0');
    }
    return card;
}