.global _start
.align 2
_start:
	// Program header
	stp x29, lr, [sp, -0x10]!
	mov x29, sp

    mov x8, 0x0a
	str x8, [sp, -0x10]!

    stp x29, lr, [sp, -0x10]!
	mov x29, sp

    mov x9, x29
    ldr x9, [x9]
    ldr x8, [x9, -0x10]
    str x8, [sp, -0x10]! 

    adrp x0, int_format@PAGE
	add x0, x0, int_format@PAGEOFF
	bl _printf

    add sp, sp, 0x10

    ldp x29, lr, [sp], 0x10

    mov x9, x29
    ldr x8, [x9, -0x10]
    str x8, [sp, -0x10]!

    adrp x0, int_format@PAGE
	add x0, x0, int_format@PAGEOFF
	bl _printf

    add sp, sp, 0x10

	// Pop the stack
	add sp, sp, 0x10
	ldp x29, lr, [sp], 0x10

	mov     x0, #0
	mov     x16, #1
	svc     0
.data
	str_format:      .asciz  "%s\n"
	int_format:      .asciz  "%d\n"
