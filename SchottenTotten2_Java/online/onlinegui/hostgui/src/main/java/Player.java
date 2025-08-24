public class Player {
    protected Hand hand;
    protected boolean usedCauldron;
    protected int cauldronCount = Constants.NUM_CAULDRONS;

    public Player() {
        hand = new Hand();
    }

    public void draw(Deck deck) {
        Card card = deck.pop();
        if (card != null) {
            hand.add(card);
        }
    }

    public Hand getHand() {
        return hand;
    }

    public boolean hasUsedCauldron() {
        return usedCauldron;
    }

    public void setUseCauldron(boolean used) {
        if (used) {
            cauldronCount--;
        }
        usedCauldron = used;
    }

    public int getCauldronCount() {
        return cauldronCount;
    }
}