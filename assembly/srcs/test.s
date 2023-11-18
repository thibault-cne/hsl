.global _start
.align 2
_start:
	stp x29, lr, [sp, #-16]!
	// Pushing variable finn to the stack
	mov x8, #10
	str x8, [sp, #-16]!
	// Pushing variable luke to the stack
	adrp x8, luke@PAGE
	add x8, x8, luke@PAGEOFF
	str x8, [sp, #-16]!
	ldr x8, [sp, #16]
	str x8, [sp, #-16]!
	adrp x0, int_format@PAGE
	add x0, x0, int_format@PAGEOFF
	bl _printf
	add sp, sp, 0x10
	ldr x8, [sp, #0]
	str x8, [sp, #-16]!
	adrp x0, str_format@PAGE
	add x0, x0, str_format@PAGEOFF
	bl _printf
	add sp, sp, 0x10
	add sp, sp, 0x20
	ldp x29, lr, [sp], #16

	mov     x0, #0
	mov     x16, #1
	svc     0
.data
	str_format:      .asciz  "%s\n"
	int_format:      .asciz  "%d\n"
	luke:      .asciz  "Luke"
