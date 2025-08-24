import java.util.*;

public class Discard {
    private static Discard instance;
    private Set<Card> discard;

    private Discard() {
        discard = new TreeSet<>();
    }

    public static Discard getInstance() {
        if (instance == null) {
            instance = new Discard();
        }
        return instance;
    }

    public void add(Card card) {
        discard.add(card);
    }

    public void addAll(Collection<Card> cards) {
        discard.addAll(cards);
    }

    public boolean contains(Card card) {
        return discard.contains(card);
    }

    public void display() {
        List<Card> list = List.copyOf(discard);
        for (int i = 0; i < list.size() - 1; i++) {
            System.out.print(list.get(i).toString() + " ");
            if (!list.get(i).getColor().equals(list.get(i + 1).getColor())) {
                System.out.println();
            }
        }
        if (!list.isEmpty()) {
            System.out.println(list.getLast());
        }
    }
}
