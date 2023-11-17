.global _start
.align 2
_start:
    stp	x29, LR, [sp, #-16]!     ; Save LR, FR
	// Pushing variable finn to the stack
	mov X1, #10
	str X1, [sp, #-16]!
	
    // Pushing variable luke to the stack
    adrp X1, luke@PAGE
    add X1, X1, luke@PAGEOFF
	str X1, [sp, #-16]!
	
    // Calling printf
	adrp X0, format_str@PAGE     // Load format_str
	add	X0, X0, format_str@PAGEOFF
	
    ldr X1, [sp]            // Load variable
    str X1, [sp, #16]!      // Push it to the stack

    bl _printf              // Call printf

    ldr X1, [sp], #16       // Empty the stack
    ldr X1, [sp], #16

    ldp	x29, LR, [sp], #16     ; Restore FR, LR

	mov     X0, #0
	mov     X16, #1
	svc     0

.data
luke:       .ascii  "Luke"
format_str: .asciz  "%s%d\n"