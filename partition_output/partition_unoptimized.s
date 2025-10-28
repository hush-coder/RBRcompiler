	.text
	.file	"test_partition.c"
	.globl	partition                       // -- Begin function partition
	.p2align	2
	.type	partition,@function
partition:                              // @partition
	.cfi_startproc
// %bb.0:
	sub	sp, sp, #48
	.cfi_def_cfa_offset 48
	str	x0, [sp, #40]
	str	w1, [sp, #36]
	str	w2, [sp, #32]
	ldr	x8, [sp, #40]
	ldrsw	x9, [sp, #32]
	ldr	w8, [x8, x9, lsl #2]
	str	w8, [sp, #28]
	ldr	w8, [sp, #36]
	subs	w8, w8, #1
	str	w8, [sp, #24]
	ldr	w8, [sp, #36]
	str	w8, [sp, #20]
	b	.LBB0_1
.LBB0_1:                                // =>This Inner Loop Header: Depth=1
	ldr	w8, [sp, #20]
	ldr	w9, [sp, #32]
	subs	w8, w8, w9
	cset	w8, ge
	tbnz	w8, #0, .LBB0_6
	b	.LBB0_2
.LBB0_2:                                //   in Loop: Header=BB0_1 Depth=1
	ldr	x8, [sp, #40]
	ldrsw	x9, [sp, #20]
	ldr	w8, [x8, x9, lsl #2]
	ldr	w9, [sp, #28]
	subs	w8, w8, w9
	cset	w8, gt
	tbnz	w8, #0, .LBB0_4
	b	.LBB0_3
.LBB0_3:                                //   in Loop: Header=BB0_1 Depth=1
	ldr	w8, [sp, #24]
	add	w8, w8, #1
	str	w8, [sp, #24]
	ldr	x8, [sp, #40]
	ldrsw	x9, [sp, #24]
	ldr	w8, [x8, x9, lsl #2]
	str	w8, [sp, #16]
	ldr	x8, [sp, #40]
	ldrsw	x9, [sp, #20]
	ldr	w8, [x8, x9, lsl #2]
	ldr	x9, [sp, #40]
	ldrsw	x10, [sp, #24]
	str	w8, [x9, x10, lsl #2]
	ldr	w8, [sp, #16]
	ldr	x9, [sp, #40]
	ldrsw	x10, [sp, #20]
	str	w8, [x9, x10, lsl #2]
	b	.LBB0_4
.LBB0_4:                                //   in Loop: Header=BB0_1 Depth=1
	b	.LBB0_5
.LBB0_5:                                //   in Loop: Header=BB0_1 Depth=1
	ldr	w8, [sp, #20]
	add	w8, w8, #1
	str	w8, [sp, #20]
	b	.LBB0_1
.LBB0_6:
	ldr	x8, [sp, #40]
	ldr	w9, [sp, #24]
	add	w9, w9, #1
	ldr	w8, [x8, w9, sxtw #2]
	str	w8, [sp, #12]
	ldr	x8, [sp, #40]
	ldrsw	x9, [sp, #32]
	ldr	w8, [x8, x9, lsl #2]
	ldr	x9, [sp, #40]
	ldr	w10, [sp, #24]
	add	w10, w10, #1
	str	w8, [x9, w10, sxtw #2]
	ldr	w8, [sp, #12]
	ldr	x9, [sp, #40]
	ldrsw	x10, [sp, #32]
	str	w8, [x9, x10, lsl #2]
	ldr	w8, [sp, #24]
	add	w0, w8, #1
	add	sp, sp, #48
	.cfi_def_cfa_offset 0
	ret
.Lfunc_end0:
	.size	partition, .Lfunc_end0-partition
	.cfi_endproc
                                        // -- End function
	.globl	generate_array                  // -- Begin function generate_array
	.p2align	2
	.type	generate_array,@function
generate_array:                         // @generate_array
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
	mov	w0, #42                         // =0x2a
	bl	srand
	str	wzr, [sp]
	b	.LBB1_1
.LBB1_1:                                // =>This Inner Loop Header: Depth=1
	ldr	w8, [sp]
	ldr	w9, [sp, #4]
	subs	w8, w8, w9
	cset	w8, ge
	tbnz	w8, #0, .LBB1_4
	b	.LBB1_2
.LBB1_2:                                //   in Loop: Header=BB1_1 Depth=1
	bl	rand
	mov	w9, #10000                      // =0x2710
	sdiv	w8, w0, w9
	mul	w8, w8, w9
	subs	w8, w0, w8
	ldr	x9, [sp, #8]
	ldrsw	x10, [sp]
	str	w8, [x9, x10, lsl #2]
	b	.LBB1_3
.LBB1_3:                                //   in Loop: Header=BB1_1 Depth=1
	ldr	w8, [sp]
	add	w8, w8, #1
	str	w8, [sp]
	b	.LBB1_1
.LBB1_4:
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]             // 16-byte Folded Reload
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
.Lfunc_end1:
	.size	generate_array, .Lfunc_end1-generate_array
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
	sub	sp, sp, #48, lsl #12            // =196608
	sub	sp, sp, #3440
	sub	x9, x29, #28
	str	x9, [sp, #8]                    // 8-byte Folded Spill
	str	wzr, [x9, #24]
	mov	w1, #50000                      // =0xc350
	str	w1, [x9, #20]
	mov	x8, sp
	stur	x8, [x9, #12]
	add	x0, sp, #20
	bl	generate_array
	ldr	x8, [sp, #8]                    // 8-byte Folded Reload
	str	wzr, [x8, #8]
	str	wzr, [x8, #4]
	b	.LBB2_1
.LBB2_1:                                // =>This Inner Loop Header: Depth=1
	ldr	x8, [sp, #8]                    // 8-byte Folded Reload
	ldr	w8, [x8, #4]
	subs	w8, w8, #100
	cset	w8, ge
	tbnz	w8, #0, .LBB2_4
	b	.LBB2_2
.LBB2_2:                                //   in Loop: Header=BB2_1 Depth=1
	add	x0, sp, #20
	str	x0, [sp]                        // 8-byte Folded Spill
	mov	w1, #50000                      // =0xc350
	bl	generate_array
	ldr	x0, [sp]                        // 8-byte Folded Reload
	mov	w1, wzr
	mov	w2, #49999                      // =0xc34f
	bl	partition
	ldr	x9, [sp, #8]                    // 8-byte Folded Reload
	str	w0, [x9]
	ldr	w10, [x9]
	ldr	w8, [x9, #8]
	add	w8, w8, w10
	str	w8, [x9, #8]
	b	.LBB2_3
.LBB2_3:                                //   in Loop: Header=BB2_1 Depth=1
	ldr	x9, [sp, #8]                    // 8-byte Folded Reload
	ldr	w8, [x9, #4]
	add	w8, w8, #1
	str	w8, [x9, #4]
	b	.LBB2_1
.LBB2_4:
	ldr	x8, [sp, #8]                    // 8-byte Folded Reload
	ldr	w9, [x8, #8]
	mov	w11, #256                       // =0x100
	sdiv	w10, w9, w11
	mul	w10, w10, w11
	subs	w9, w9, w10
	str	w9, [x8, #24]
	ldur	x9, [x8, #12]
	mov	sp, x9
	ldr	w0, [x8, #24]
	add	sp, sp, #48, lsl #12            // =196608
	add	sp, sp, #3440
	.cfi_def_cfa wsp, 32
	ldr	x28, [sp, #16]                  // 8-byte Folded Reload
	ldp	x29, x30, [sp], #32             // 16-byte Folded Reload
	.cfi_def_cfa_offset 0
	.cfi_restore w28
	.cfi_restore w30
	.cfi_restore w29
	ret
.Lfunc_end2:
	.size	main, .Lfunc_end2-main
	.cfi_endproc
                                        // -- End function
	.ident	"BiSheng Enterprise 4.2.0.2.B002 clang version 17.0.6 (2261d9fde4e0)"
	.section	".note.GNU-stack","",@progbits
	.addrsig
	.addrsig_sym partition
	.addrsig_sym generate_array
	.addrsig_sym srand
	.addrsig_sym rand
