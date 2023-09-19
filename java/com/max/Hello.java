package com.max;

public class Hello {

    public static void main(String[] args) {
        int first = fib();
        int second = fib();

        int res = first + second;      
    }

    public static int fib(){
        int a = 1;
        int b = 1;
        for(int i = 0; i < 10; ++i){
            int temp = a + b;
            a = b;
            b = temp;
        } 

        return a;
    }
}
