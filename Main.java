package Main;

public class Main {
    static int sa = 10;
    static {
        sa = 2;
    }
    public static void main(String[] args) {
        int a = 1000000;
	String x = "ssss";
	int b = 10;
	int c = a + b;
	int d = add(a, b);
    }

    static int add(int a, int b) {
        return a + b;
    }
}
