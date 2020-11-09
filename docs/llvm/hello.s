	.section	__TEXT,__text,regular,pure_instructions
	.macosx_version_min 10, 15	sdk_version 10, 15, 6
	.globl	_main                   ## -- Begin function main
	.p2align	4, 0x90
_main:                                  ## @main
	.cfi_startproc
## %bb.0:
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	leaq	L_.str(%rip), %rdi
	movb	$0, %al
	callq	_printf
	xorl	%eax, %eax
	popq	%rbp
	retq
	.cfi_endproc
                                        ## -- End function
	.section	__TEXT,__cstring,cstring_literals
L_.str:                                 ## @.str
	.asciz	"Hello World!\n"

.subsections_via_symbols
