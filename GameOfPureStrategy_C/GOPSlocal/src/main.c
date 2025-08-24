#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include "deck.h"
#include "player.h"
#include "card.h"
#include "prize.h"

Card getCardInput() {
    printf("Enter card: ");
    fflush(stdout);
    char s[5];
    scanf("%2s", s);
    return stringToCard(s);
}

void runGame(Player *player1, Player *player2, Deck *deck) {
    Prize prizes;
    initPrizePool(&prizes);
    while (!deckIsEmpty(deck)) {
        addPrizeCard(&prizes, popDeck(deck));
        printPrizes(&prizes);
        fflush(stdout);

        printf("Player 1\n");
        displayHand(player1);
        fflush(stdout);
        Card c1 = getCardInput();
        while (!removeCard(player1, c1)) {
            printf("you don't have that card\n");
            c1 = getCardInput();
        }

        printf("Player 2\n");
        displayHand(player2);
        fflush(stdout);
        Card c2 = getCardInput();
        while (!removeCard(player2, c2)) {
            printf("you don't have that card\n");
            c2 = getCardInput();
        }

        int diff = compareCards(&c1, &c2);
        if (diff > 0) {
            addPoints(player1, totalPrizeValue(&prizes));
        } else if (diff < 0) {
            addPoints(player2, totalPrizeValue(&prizes));
        } else {
            continue;
        }

        clearPrizes(&prizes);
        printf("Player 1 has %d points\n", player1->points);
        printf("Player 2 has %d points\n", player2->points);
        printf("\n");
        fflush(stdout);
    }

    if (player1->points > player2->points) {
        printf("Player 1 wins!");
    } else if (player1->points < player2->points) {
        printf("Player 2 wins!");
    } else {
        printf("Tie!");
    }

    freePrizePool(&prizes);
}

int main() {
    srand(time(NULL));

    Deck deck;
    initDeck(&deck);
    shuffleDeck(&deck);

    Player player1;
    initPlayer(&player1);
    Player player2;
    initPlayer(&player2);

    runGame(&player1, &player2, &deck);

    return 0;
}