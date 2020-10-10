
# riscv64-unknown-elf-gcc -Wl,-Ttext=0x0 -nostdlib -o add-addi add-addi.s
# riscv64-unknown-elf-objcopy -O binary add-addi add-addi.bin

main:
    addi x29, x0, 5
    addi x30, x0, 37

    add  x31, x30, x29
