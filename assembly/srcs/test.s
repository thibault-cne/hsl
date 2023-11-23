.global _start
.align 2
_start:
	stp x29, lr, [sp, -0x10]!
	mov x29, sp
	// Pushing variable finn to the stack
	mov x8, #10
	str x8, [sp, -0x10]!
	ldr x8, [x29, -0x10]
	str x8, [sp, -0x10]!
	adrp x0, int_format@PAGE
	add x0, x0, int_format@PAGEOFF
	bl _printf
	add sp, sp, 0x10
	adrp x0, str_format@PAGE
	add x0, x0, str_format@PAGEOFF
	adrp x8, _lit_2@PAGE
	add x8, x8, _lit_2@PAGEOFF
	str x8, [sp, -0x10]!
	bl _printf
	add sp, sp, 0x10
	mov x8, 0xa
	str x8, [sp, -0x10]!
	adrp x0, int_format@PAGE
	add x0, x0, int_format@PAGEOFF
	bl _printf
	add sp, sp, 0x10
	add sp, sp, 0x10
	ldp x29, lr, [sp], 0x10

	mov     x0, #0
	mov     x16, #1
	svc     0
.data
	str_format:      .asciz  "%s\n"
	int_format:      .asciz  "%d\n"
	_lit_2:      .asciz  "Do or do not, there is no try!"
