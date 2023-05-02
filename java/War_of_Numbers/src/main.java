import java.util.Vector;

public class main {

    static int aToI (String aNum){
        int num = 0;
        for(byte c : aNum.getBytes()) {
            num *= 10;
            if(c >= '0' && c <= '9')
                num += (c - '0');
            else throw new IllegalArgumentException("String is not purely a positive number");
        }

        return num;
    }

    static int[] argsToArray(String[] args){
        int[] array = new int[args.length];
        for(int i = 0; i < args.length; ++i) {
            array[i] = aToI(args[i]);
        }
        return array;
    }

    public static void challengeFunction(int[] array){
        int odd = 0, even = odd;
        for(int num: array) {
            if( (num & 1) !=0)
                odd += num;
            else
                even += num;
        }

        System.out.format("%s is greater\n", odd > even ? "odd" : "even");
        System.out.format("event total: %d; odd total: %d\n", even, odd);
        System.out.format("%d is the difference\n", odd - even < 0 ? even-odd : odd-even);
    }

    public static void main(String[] args) {
        int[] array = argsToArray(args);
        challengeFunction(array);
        System.exit(0);
    }
}
