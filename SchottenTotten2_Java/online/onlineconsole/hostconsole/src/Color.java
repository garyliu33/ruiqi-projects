public enum Color {
    RED("red", "♥️", "heart", "apple"),
    BLUE("blue", "\uD83D\uDC8E", "diamond", "grape"),
    YELLOW("yellow", "⭐️", "star", "lemon"),
    GREEN("green", "\uD83C\uDF40", "clover", "mango"),
    PINK("pink", "\uD83C\uDF38", "flower", "peach");

    private final String color;
    private final String emoji;
    private final String name;
    private final String fruit;

    Color(String color, String emoji, String name, String fruit) {
        this.color = color;
        this.emoji = emoji;
        this.name = name;
        this.fruit = fruit;
    }

    public String getSymbol() {
        return Host.useEmojis ? emoji : fruit;
    }

    public static Color convert(String str) {
        str = str.trim().toLowerCase();
        for (Color color : values()) {
            if (str.equals(color.color) || str.equals(color.emoji) || str.equals(color.name) || str.equals(color.fruit)) {
                return color;
            }
        }
        return null;
    }

    public enum ColorType {
        COLOR, EMOJI, NAME, FRUIT
    }

    public static String listOf(ColorType type) {
        StringBuilder str = new StringBuilder();
        for (Color color : Color.values()) {
            switch(type) {
                case COLOR -> {str.append(color.color);}
                case EMOJI -> {str.append(color.emoji);}
                case NAME -> {str.append(color.name);}
                case FRUIT -> {str.append(color.fruit);}
            }
            str.append(", ");
        }

        return str.substring(0, str.length() - 2);
    }
}

