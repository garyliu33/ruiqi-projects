#include "deck.h"
#include <stdlib.h>

void addCardToDeck(Deck *deck, Card card) {
    if (deck->size < NUM_CARDS) {
        deck->cards[deck->size] = card;
        deck->size++;
    }
}

void initDeck(Deck *deck) {
    deck->size = 0;
    for (int i = 1; i <= NUM_CARDS; i++ ) {
        Card card = createCard(i);
        addCardToDeck(deck, card);
    }
}

void shuffleDeck(Deck *deck) {
    for (int i = deck->size - 1; i >= 0; i--) {
        int j = rand() % (i + 1);
        Card temp = deck->cards[i];
        deck->cards[i] = deck->cards[j];
        deck->cards[j] = temp;
    }
}

Card popDeck(Deck *deck) {
    if (deck->size > 0) {
        deck->size--;
        return deck->cards[deck->size];
    }
}

int deckIsEmpty(Deck *deck) {
    return deck->size == 0;
}