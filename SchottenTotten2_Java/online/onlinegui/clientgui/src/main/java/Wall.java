import java.util.ArrayList;
import java.util.List;

public class Wall {
    private final int wallIndex;
    private final Status status;
    private final int length;
    private final WallPattern pattern;

    private final List<Card> attackerCards;
    private final List<Card> defenderCards;

    public enum Status {
        BROKEN, DAMAGED, INTACT
    }

    public Wall(int wallIndex, int intactLength, WallPattern intactPattern) {
        this.wallIndex = wallIndex;
        this.status = Status.INTACT;
        this.length = intactLength;
        this.pattern = intactPattern;

        this.attackerCards = new ArrayList<>();
        this.defenderCards = new ArrayList<>();
    }

    public List<Card> getAttackerCards() {
        return attackerCards;
    }

    public List<Card> getDefenderCards() {
        return defenderCards;
    }

    public Status getStatus() {
        return status;
    }

    public WallPattern getPattern() {
        return pattern;
    }

    public int getLength() {
        return length;
    }

    public int getWallIndex() {
        return wallIndex;
    }
}
