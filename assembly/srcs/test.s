.global _start
.align 2
_start:
	stp x29, lr, [sp, -0x10]!
	mov x29, sp
	// Pushing variable finn to the stack
	mov x8, #10
	str x8, [sp, -0x10]!
	// Pushing variable luke to the stack
	adrp x8, luke@PAGE
	add x8, x8, luke@PAGEOFF
	str x8, [sp, -0x10]!
	// Pushing variable poe to the stack
	adrp x8, poe@PAGE
	add x8, x8, poe@PAGEOFF
	str x8, [sp, -0x10]!
	// Pushing variable first_order to the stack
	adrp x8, first_order@PAGE
	add x8, x8, first_order@PAGEOFF
	str x8, [sp, -0x10]!
	ldr x8, [x29, -0x40]
	str x8, [sp, -0x10]!
	adrp x0, str_format@PAGE
	add x0, x0, str_format@PAGEOFF
	bl _printf
	add sp, sp, 0x10
	ldr x8, [x29, -0x10]
	str x8, [sp, -0x10]!
	adrp x0, int_format@PAGE
	add x0, x0, int_format@PAGEOFF
	bl _printf
	add sp, sp, 0x10
	ldr x8, [x29, -0x20]
	str x8, [sp, -0x10]!
	adrp x0, str_format@PAGE
	add x0, x0, str_format@PAGEOFF
	bl _printf
	add sp, sp, 0x10
	ldr x8, [x29, -0x30]
	str x8, [sp, -0x10]!
	adrp x0, str_format@PAGE
	add x0, x0, str_format@PAGEOFF
	bl _printf
	add sp, sp, 0x10
	add sp, sp, 0x40
	ldp x29, lr, [sp], 0x10

	mov     x0, #0
	mov     x16, #1
	svc     0
.data
	str_format:      .asciz  "%s\n"
	int_format:      .asciz  "%d\n"
	luke:      .asciz  "Luke"
	poe:      .asciz  "Poe Dameron"
	first_order:      .asciz  "This is the first order"
