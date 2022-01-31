import java.util.*;
public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int n = sc.nextInt();
        int[] a = new int[n];
        for (int i = 0; i < n; i++) {
            a[i] = sc.nextInt();
        }
        int cnt = 0;
        int id = 0;
        int rec[] = new int[9];
        for (int i = 0; i < n; i++) {
            rec[a[i] - 1] += 1;
        }
        for (int i = 0; i < 9; i+= 1) {
            if(cnt < rec[i]) {
                cnt = rec[i];
                id = i + 1;
            }
        }
        System.out.println(id);
    }
}