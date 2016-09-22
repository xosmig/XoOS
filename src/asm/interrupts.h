// FIXME: TODO: use rust naked functions instead of pure asm. See: http://os.phil-opp.com/returning-from-exceptions.html

// fix the difference between two handlers for easiest settings
#define INTERRUPT_HANDLER_DIFF 64

// scratch registers: rax, rcx, rdx, rsi, rdi, r8, r9, r10, r11

#define PUSH_ENVIRONMENT \
    pushq %rax; \
    pushq %rcx; \
    pushq %rdx; \
    pushq %rsi; \
    pushq %rdi; \
    pushq %r8;  \
    pushq %r9;  \
    pushq %r10; \
    pushq %r11;

#define POP_ENVIRONMENT \
    popq %r11; \
    popq %r10; \
    popq %r9;  \
    popq %r8;  \
    popq %rdi; \
    popq %rsi; \
    popq %rdx; \
    popq %rcx; \
    popq %rax;


#define MAKE_INTERRUPT_HANDLER(num) \
interrupt##num: \
    PUSH_ENVIRONMENT; \
    movq $0, %rsi; \
    movb $num, %dil; \
    cld; \
    callq handle_interrupt; \
    POP_ENVIRONMENT; \
    iretq; \
    . = interrupt##num + INTERRUPT_HANDLER_DIFF

#define MAKE_EXCEPTION_HANDLER(num) \
interrupt##num: \
    PUSH_ENVIRONMENT; \
    movq 0x70(%rsp), %rsi; \
    movb $num, %dil; \
    cld; \
    callq handle_interrupt; \
    POP_ENVIRONMENT; \
    add $0x8, %rsp; \
    iretq; \
    . = interrupt##num + INTERRUPT_HANDLER_DIFF
