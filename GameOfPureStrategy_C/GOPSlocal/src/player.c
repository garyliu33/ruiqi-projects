#include "player.h"
#include "card.h"
#include <stdio.h>

void initPlayer(Player *player) {
    player->handSize = 0;
    player->points = 0;
    for (int i = 1; i <= NUM_CARDS; i++) {
        Card card = createCard(i);
        addCard(player, card);
    }
}

int removeCard(Player *player, Card card) {
    int index = indexOf(player, card);
    if (index < 0 || index >= player->handSize) {
        return 0;
    }
    for (int i = index; i < player->handSize - 1; i++) {
        player->hand[i] = player->hand[i + 1];
    }
    player->handSize--;
    return 1;
}

void addCard(Player *player, Card card) {
    if (player->handSize < NUM_CARDS) {
        player->hand[player->handSize] = card;
        player->handSize++;
    }
}

int indexOf(Player *player, Card card) {
    for (int i = 0; i < player->handSize; i++) {
        if (card.rank == player->hand[i].rank) {
            return i;
        }
    }
    return -1;
}

void displayHand(Player *player) {
    for (int i = 0; i < player->handSize; i++) {
        printCard(&player->hand[i]);
        printf(" ");
    }
    printf("\n");
}

void addPoints(Player *player, int points) {
    player->points += points;
}