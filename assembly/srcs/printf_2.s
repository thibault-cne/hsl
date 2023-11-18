.global _start
.align 2

_start:
    stp	x29, LR, [sp, #-16]!     ; Save LR, FR
	sub sp, sp, 0x10

    adrp x8, luke@PAGE
    add x8, x8, luke@PAGEOFF
    str x8, [sp]
    
    adrp x0, format_str@PAGE
    add x0, x0, format_str@PAGEOFF
    bl _printf

    add sp, sp, 0x10
    ldp	x29, LR, [sp], #16     ; Restore FR, LR

	mov     X0, #0
	mov     X16, #1
	svc     0

.data
    luke:       .asciz  "Luke"
    format_str: .asciz  "%s\n"