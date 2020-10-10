
/*
riscv64-unknown-elf-gcc -S fib.c && \
riscv64-unknown-elf-gcc -Wl,-Ttext=0x0 -nostdlib -o fib fib.s && \
riscv64-unknown-elf-objcopy -O binary fib fib.bin
*/


static int fib(int n);


int main()
{
    return fib(10);
}


static int fib(int n)
{
    if ((n == 0) || (n == 1))
    {
        return n;
    }

    return fib(n - 1) + fib(n - 2);
}
