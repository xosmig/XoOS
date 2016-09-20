// fix the difference between two handlers for easiest settings
#define INTERRUPT_HANDLER_DIFF 64

#define PUSH_ENVIRONMENT \
    pushq %rax; \
    pushq %rbx; \
    pushq %rcx; \
    pushq %rdx; \
    pushq %rbp; \
    pushq %rdi; \
    pushq %rsi; \
    pushq %r9;  \
    pushq %r10; \
    pushq %r11; \
    pushq %r12; \
    pushq %r13; \
    pushq %r14; \
    pushq %r15;

//    RAX, RBX, RCX, RDX, RBP, RDI, RSI, R9 -
//          R15

#define POP_ENVIRONMENT \
    popq %r15; \
    popq %r14; \
    popq %r13; \
    popq %r12; \
    popq %r11; \
    popq %r10; \
    popq %r9;  \
    popq %rsi; \
    popq %rdi; \
    popq %rbp; \
    popq %rdx; \
    popq %rcx; \
    popq %rbx; \
    popq %rax; \
    iretq;

#define MAKE_INTERRUPT_HANDLER(num) \
interrupt##num: \
    PUSH_ENVIRONMENT; \
	movb $num, %dil; \
	callq handle_interrupt; \
    POP_ENVIRONMENT; \
    . = interrupt##num + INTERRUPT_HANDLER_DIFF
