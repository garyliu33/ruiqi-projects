#ifndef PRIZE_H
#define PRIZE_H
#include "card.h"

typedef struct {
    Card* cards;
    int size;
} Prize;

void initPrizePool(Prize *prize);
void addPrizeCard(Prize *prize, Card card);
void clearPrizes(Prize *prize);
void printPrizes(Prize *prize);
void freePrizePool(Prize *prize);
int totalPrizeValue(Prize *prize);

#endif //PRIZE_H
