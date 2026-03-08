.option norvc

.section .text.init
.global _start
_start:
    # Park all harts except hart 0
    csrr    t0, mhartid
    bnez    t0, park

    # Set up the stack pointer
    la      sp, _stack_end

    # Zero out BSS
    la      t0, _bss_start
    la      t1, _bss_end
bss_clear:
    bgeu    t0, t1, bss_done
    sd      zero, 0(t0)
    addi    t0, t0, 8
    j       bss_clear
bss_done:

    # Jump to Rust
    j       kmain

park:
    wfi
    j       park

