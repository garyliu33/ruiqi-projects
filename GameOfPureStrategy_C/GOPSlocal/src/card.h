#ifndef CARD_H
#define CARD_H
#define NUM_CARDS 13

typedef struct {
    int rank;
} Card;

Card createCard(int rank);
int compareCards(Card *a, Card *b);
void printCard(Card *card);
Card stringToCard(char *s);

#endif //CARD_H