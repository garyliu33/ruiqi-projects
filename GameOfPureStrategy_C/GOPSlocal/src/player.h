#ifndef PLAYER_H
#define PLAYER_H
#include "deck.h"

typedef struct {
    Card hand[NUM_CARDS];
    int handSize;
    int points;
} Player;

void initPlayer(Player *player);
int removeCard(Player *player, Card card);
void addCard(Player *player, Card card);
int indexOf(Player *player, Card card);
void displayHand(Player *player);
void addPoints(Player *player, int points);

#endif //PLAYER_H
