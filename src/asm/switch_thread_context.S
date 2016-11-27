    .text
    .global switch_thread_context

switch_thread_context:
    #define     FLAGS_TO_CHANGE 0b11010101
    #define NOT_FLAGS_TO_CHANGE (~FLAGS_TO_CHANGE)

    // save context on stack
    pushq %rbx
    pushq %rbp
    pushq %r12
    pushq %r13
    pushq %r14
    pushq %r15
    pushfq
    mov (%rsp), %rbx                 // current flag register
    andq $NOT_FLAGS_TO_CHANGE, %rbx  // forget state flags

    movq %rsp, (%rdi)
    movq %rsi, %rsp

    popq %rbp  // new flag register

    // count new flag register
    andq $FLAGS_TO_CHANGE, %rbp  // get only state flags
    orq %rbx, %rbp

    // update flag register
    pushq %rbp
    popfq

    // load the rest of context
    popq %r15
    popq %r14
    popq %r13
    popq %r12
    popq %rbp
    popq %rbx

    sti  // unlock interrupts
    ret