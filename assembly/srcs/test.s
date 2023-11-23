.global _start
.align 2
_start:
	// Program header
	stp x29, lr, [sp, -0x10]!
	mov x29, sp

	// Jump to main function
	b _main
_start_end:
	// Pop the stack
	add sp, sp, 0x10
	ldp x29, lr, [sp], 0x10

	mov     x0, #0
	mov     x16, #1
	svc     0

_main:
	// Pushing variable finn to the stack
	mov x8, -0xa
	str x8, [sp, -0x10]!
	// Start operations
	mov x8, 0x0
	add x8, x8, 0x1
	str x8, [x29, -0x10]
	// End operations
	// Load variable finn
	mov x9, x29
	ldr x8, [x9, -0x10]
	cmp x8, 0x0
	bne _if_else_2

	// Start of then block
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

	// Unstack then block
	add sp, sp, 0x0
	ldp x29, lr, [sp], 0x10
	b _if_end_2

_if_else_2:
	// Start of else block
	stp x29, lr, [sp, -0x10]!
	mov x29, sp
	adrp x8, _lit_2@PAGE
	add x8, x8, _lit_2@PAGEOFF
	adrp x0, str_format@PAGE
	add x0, x0, str_format@PAGEOFF
	str x8, [sp, -0x10]!
	bl _printf
	add sp, sp, 0x10

	// Unstack else block
	add sp, sp, 0x0
	ldp x29, lr, [sp], 0x10

_if_end_2:
	// Start operations
	mov x8, 0x0
	str x8, [x29, -0x10]
	// End operations
	// Load variable finn
	mov x9, x29
	ldr x8, [x9, -0x10]
	cmp x8, 0x0
	bne _if_else_4

	// Start of then block
	stp x29, lr, [sp, -0x10]!
	mov x29, sp
	adrp x8, _lit_3@PAGE
	add x8, x8, _lit_3@PAGEOFF
	adrp x0, str_format@PAGE
	add x0, x0, str_format@PAGEOFF
	str x8, [sp, -0x10]!
	bl _printf
	add sp, sp, 0x10

	// Unstack then block
	add sp, sp, 0x0
	ldp x29, lr, [sp], 0x10
	b _if_end_4

_if_else_4:
	// Start of else block
	stp x29, lr, [sp, -0x10]!
	mov x29, sp
	adrp x8, _lit_4@PAGE
	add x8, x8, _lit_4@PAGEOFF
	adrp x0, str_format@PAGE
	add x0, x0, str_format@PAGEOFF
	str x8, [sp, -0x10]!
	bl _printf
	add sp, sp, 0x10

	// Unstack else block
	add sp, sp, 0x0
	ldp x29, lr, [sp], 0x10

_if_end_4:
	// Jump to end of program
	b _start_end
.data
	str_format:      .asciz  "%s\n"
	int_format:      .asciz  "%d\n"
	_lit_2:      .asciz  "Don't"
	_lit_3:      .asciz  "Do"
	_lit_4:      .asciz  "Don't"
