#ifndef DECK_H
#define DECK_H

#include "card.h"

typedef struct {
    Card cards[NUM_CARDS];
    int size;
} Deck;

void addCardToDeck(Deck *deck, Card card);
void initDeck(Deck *deck);
void shuffleDeck(Deck *deck);
Card popDeck(Deck *deck);
int deckIsEmpty(Deck *deck);

#endif //DECK_H
