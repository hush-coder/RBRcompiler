	.text
	.file	"quicksort.c"
	.globl	swap                            // -- Begin function swap
	.p2align	2
	.type	swap,@function
swap:                                   // @swap
	.cfi_startproc
// %bb.0:
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	str	x0, [sp, #24]
	str	x1, [sp, #16]
	ldr	x8, [sp, #24]
	ldr	w8, [x8]
	str	w8, [sp, #12]
	ldr	x8, [sp, #16]
	ldr	w8, [x8]
	ldr	x9, [sp, #24]
	str	w8, [x9]
	ldr	w8, [sp, #12]
	ldr	x9, [sp, #16]
	str	w8, [x9]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	ret
.Lfunc_end0:
	.size	swap, .Lfunc_end0-swap
	.cfi_endproc
                                        // -- End function
	.globl	partition                       // -- Begin function partition
	.p2align	2
	.type	partition,@function
partition:                              // @partition
	.cfi_startproc
// %bb.0:
	sub	sp, sp, #48
	.cfi_def_cfa_offset 48
	stp	x29, x30, [sp, #32]             // 16-byte Folded Spill
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	stur	x0, [x29, #-8]
	stur	w1, [x29, #-12]
	str	w2, [sp, #16]
	ldur	x8, [x29, #-8]
	ldrsw	x9, [sp, #16]
	ldr	w8, [x8, x9, lsl #2]
	str	w8, [sp, #12]
	ldur	w8, [x29, #-12]
	subs	w8, w8, #1
	str	w8, [sp, #8]
	ldur	w8, [x29, #-12]
	str	w8, [sp, #4]
	b	.LBB1_1
.LBB1_1:                                // =>This Inner Loop Header: Depth=1
	ldr	w8, [sp, #4]
	ldr	w9, [sp, #16]
	subs	w9, w9, #1
	subs	w8, w8, w9
	cset	w8, gt
	tbnz	w8, #0, .LBB1_6
	b	.LBB1_2
.LBB1_2:                                //   in Loop: Header=BB1_1 Depth=1
	ldur	x8, [x29, #-8]
	ldrsw	x9, [sp, #4]
	ldr	w8, [x8, x9, lsl #2]
	ldr	w9, [sp, #12]
	subs	w8, w8, w9
	cset	w8, gt
	tbnz	w8, #0, .LBB1_4
	b	.LBB1_3
.LBB1_3:                                //   in Loop: Header=BB1_1 Depth=1
	ldr	w8, [sp, #8]
	add	w8, w8, #1
	str	w8, [sp, #8]
	ldur	x8, [x29, #-8]
	ldrsw	x9, [sp, #8]
	add	x0, x8, x9, lsl #2
	ldur	x8, [x29, #-8]
	ldrsw	x9, [sp, #4]
	add	x1, x8, x9, lsl #2
	bl	swap
	b	.LBB1_4
.LBB1_4:                                //   in Loop: Header=BB1_1 Depth=1
	b	.LBB1_5
.LBB1_5:                                //   in Loop: Header=BB1_1 Depth=1
	ldr	w8, [sp, #4]
	add	w8, w8, #1
	str	w8, [sp, #4]
	b	.LBB1_1
.LBB1_6:
	ldur	x8, [x29, #-8]
	ldr	w9, [sp, #8]
	add	w9, w9, #1
	add	x0, x8, w9, sxtw #2
	ldur	x8, [x29, #-8]
	ldrsw	x9, [sp, #16]
	add	x1, x8, x9, lsl #2
	bl	swap
	ldr	w8, [sp, #8]
	add	w0, w8, #1
	.cfi_def_cfa wsp, 48
	ldp	x29, x30, [sp, #32]             // 16-byte Folded Reload
	add	sp, sp, #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
.Lfunc_end1:
	.size	partition, .Lfunc_end1-partition
	.cfi_endproc
                                        // -- End function
	.globl	quicksort                       // -- Begin function quicksort
	.p2align	2
	.type	quicksort,@function
quicksort:                              // @quicksort
	.cfi_startproc
// %bb.0:
	sub	sp, sp, #48
	.cfi_def_cfa_offset 48
	stp	x29, x30, [sp, #32]             // 16-byte Folded Spill
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	stur	x0, [x29, #-8]
	stur	w1, [x29, #-12]
	str	w2, [sp, #16]
	ldur	w8, [x29, #-12]
	ldr	w9, [sp, #16]
	subs	w8, w8, w9
	cset	w8, ge
	tbnz	w8, #0, .LBB2_2
	b	.LBB2_1
.LBB2_1:
	ldur	x0, [x29, #-8]
	ldur	w1, [x29, #-12]
	ldr	w2, [sp, #16]
	bl	partition
	str	w0, [sp, #12]
	ldur	x0, [x29, #-8]
	ldur	w1, [x29, #-12]
	ldr	w8, [sp, #12]
	subs	w2, w8, #1
	bl	quicksort
	ldur	x0, [x29, #-8]
	ldr	w8, [sp, #12]
	add	w1, w8, #1
	ldr	w2, [sp, #16]
	bl	quicksort
	b	.LBB2_2
.LBB2_2:
	.cfi_def_cfa wsp, 48
	ldp	x29, x30, [sp, #32]             // 16-byte Folded Reload
	add	sp, sp, #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
.Lfunc_end2:
	.size	quicksort, .Lfunc_end2-quicksort
	.cfi_endproc
                                        // -- End function
	.globl	print_array                     // -- Begin function print_array
	.p2align	2
	.type	print_array,@function
print_array:                            // @print_array
	.cfi_startproc
// %bb.0:
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]             // 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	w1, [sp, #4]
	str	wzr, [sp]
	b	.LBB3_1
.LBB3_1:                                // =>This Inner Loop Header: Depth=1
	ldr	w8, [sp]
	ldr	w9, [sp, #4]
	subs	w8, w8, w9
	cset	w8, ge
	tbnz	w8, #0, .LBB3_4
	b	.LBB3_2
.LBB3_2:                                //   in Loop: Header=BB3_1 Depth=1
	ldr	x8, [sp, #8]
	ldrsw	x9, [sp]
	ldr	w1, [x8, x9, lsl #2]
	adrp	x0, .L.str
	add	x0, x0, :lo12:.L.str
	bl	printf
	b	.LBB3_3
.LBB3_3:                                //   in Loop: Header=BB3_1 Depth=1
	ldr	w8, [sp]
	add	w8, w8, #1
	str	w8, [sp]
	b	.LBB3_1
.LBB3_4:
	adrp	x0, .L.str.1
	add	x0, x0, :lo12:.L.str.1
	bl	printf
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]             // 16-byte Folded Reload
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
.Lfunc_end3:
	.size	print_array, .Lfunc_end3-print_array
	.cfi_endproc
                                        // -- End function
	.globl	generate_random_array           // -- Begin function generate_random_array
	.p2align	2
	.type	generate_random_array,@function
generate_random_array:                  // @generate_random_array
	.cfi_startproc
// %bb.0:
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]             // 16-byte Folded Spill
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	w1, [sp, #4]
	mov	x0, xzr
	bl	time
                                        // kill: def $w0 killed $w0 killed $x0
	bl	srand
	str	wzr, [sp]
	b	.LBB4_1
.LBB4_1:                                // =>This Inner Loop Header: Depth=1
	ldr	w8, [sp]
	ldr	w9, [sp, #4]
	subs	w8, w8, w9
	cset	w8, ge
	tbnz	w8, #0, .LBB4_4
	b	.LBB4_2
.LBB4_2:                                //   in Loop: Header=BB4_1 Depth=1
	bl	rand
	mov	w9, #1000                       // =0x3e8
	sdiv	w8, w0, w9
	mul	w8, w8, w9
	subs	w8, w0, w8
	ldr	x9, [sp, #8]
	ldrsw	x10, [sp]
	str	w8, [x9, x10, lsl #2]
	b	.LBB4_3
.LBB4_3:                                //   in Loop: Header=BB4_1 Depth=1
	ldr	w8, [sp]
	add	w8, w8, #1
	str	w8, [sp]
	b	.LBB4_1
.LBB4_4:
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]             // 16-byte Folded Reload
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
.Lfunc_end4:
	.size	generate_random_array, .Lfunc_end4-generate_random_array
	.cfi_endproc
                                        // -- End function
	.globl	is_sorted                       // -- Begin function is_sorted
	.p2align	2
	.type	is_sorted,@function
is_sorted:                              // @is_sorted
	.cfi_startproc
// %bb.0:
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	str	x0, [sp, #16]
	str	w1, [sp, #12]
	mov	w8, #1                          // =0x1
	str	w8, [sp, #8]
	b	.LBB5_1
.LBB5_1:                                // =>This Inner Loop Header: Depth=1
	ldr	w8, [sp, #8]
	ldr	w9, [sp, #12]
	subs	w8, w8, w9
	cset	w8, ge
	tbnz	w8, #0, .LBB5_6
	b	.LBB5_2
.LBB5_2:                                //   in Loop: Header=BB5_1 Depth=1
	ldr	x8, [sp, #16]
	ldrsw	x9, [sp, #8]
	ldr	w8, [x8, x9, lsl #2]
	ldr	x9, [sp, #16]
	ldr	w10, [sp, #8]
	subs	w10, w10, #1
	ldr	w9, [x9, w10, sxtw #2]
	subs	w8, w8, w9
	cset	w8, ge
	tbnz	w8, #0, .LBB5_4
	b	.LBB5_3
.LBB5_3:
	str	wzr, [sp, #28]
	b	.LBB5_7
.LBB5_4:                                //   in Loop: Header=BB5_1 Depth=1
	b	.LBB5_5
.LBB5_5:                                //   in Loop: Header=BB5_1 Depth=1
	ldr	w8, [sp, #8]
	add	w8, w8, #1
	str	w8, [sp, #8]
	b	.LBB5_1
.LBB5_6:
	mov	w8, #1                          // =0x1
	str	w8, [sp, #28]
	b	.LBB5_7
.LBB5_7:
	ldr	w0, [sp, #28]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	ret
.Lfunc_end5:
	.size	is_sorted, .Lfunc_end5-is_sorted
	.cfi_endproc
                                        // -- End function
	.globl	main                            // -- Begin function main
	.p2align	2
	.type	main,@function
main:                                   // @main
	.cfi_startproc
// %bb.0:
	stp	x29, x30, [sp, #-32]!           // 16-byte Folded Spill
	.cfi_def_cfa_offset 32
	str	x28, [sp, #16]                  // 8-byte Folded Spill
	mov	x29, sp
	.cfi_def_cfa w29, 32
	.cfi_offset w28, -16
	.cfi_offset w30, -24
	.cfi_offset w29, -32
	sub	sp, sp, #97, lsl #12            // =397312
	sub	sp, sp, #2736
	sub	x9, x29, #16
	str	x9, [sp, #24]                   // 8-byte Folded Spill
	mov	w1, #34464                      // =0x86a0
	movk	w1, #1, lsl #16
	str	w1, [sp, #20]                   // 4-byte Folded Spill
	mov	w8, wzr
	str	w8, [sp, #4]                    // 4-byte Folded Spill
	str	wzr, [x9, #12]
	str	w1, [x9, #8]
	mov	x8, sp
	str	x8, [x9]
	add	x0, sp, #32
	str	x0, [sp, #8]                    // 8-byte Folded Spill
	bl	generate_random_array
	ldr	w1, [sp, #4]                    // 4-byte Folded Reload
	ldr	x0, [sp, #8]                    // 8-byte Folded Reload
	mov	w2, #34463                      // =0x869f
	movk	w2, #1, lsl #16
	bl	quicksort
	ldr	x0, [sp, #8]                    // 8-byte Folded Reload
	ldr	w1, [sp, #20]                   // 4-byte Folded Reload
	bl	is_sorted
	ldr	x8, [sp, #24]                   // 8-byte Folded Reload
	subs	w9, w0, #0
	cset	w9, ne
	and	w9, w9, #0x1
	ands	w9, w9, #0x1
	cset	w9, eq
	str	w9, [x8, #12]
	ldr	x9, [x8]
	mov	sp, x9
	ldr	w0, [x8, #12]
	add	sp, sp, #97, lsl #12            // =397312
	add	sp, sp, #2736
	.cfi_def_cfa wsp, 32
	ldr	x28, [sp, #16]                  // 8-byte Folded Reload
	ldp	x29, x30, [sp], #32             // 16-byte Folded Reload
	.cfi_def_cfa_offset 0
	.cfi_restore w28
	.cfi_restore w30
	.cfi_restore w29
	ret
.Lfunc_end6:
	.size	main, .Lfunc_end6-main
	.cfi_endproc
                                        // -- End function
	.type	.L.str,@object                  // @.str
	.section	.rodata.str1.1,"aMS",@progbits,1
.L.str:
	.asciz	"%d "
	.size	.L.str, 4

	.type	.L.str.1,@object                // @.str.1
.L.str.1:
	.asciz	"\n"
	.size	.L.str.1, 2

	.ident	"BiSheng Enterprise 4.2.0.2.B002 clang version 17.0.6 (2261d9fde4e0)"
	.section	".note.GNU-stack","",@progbits
	.addrsig
	.addrsig_sym swap
	.addrsig_sym partition
	.addrsig_sym quicksort
	.addrsig_sym printf
	.addrsig_sym generate_random_array
	.addrsig_sym srand
	.addrsig_sym time
	.addrsig_sym rand
	.addrsig_sym is_sorted
