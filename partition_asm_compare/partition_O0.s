	.text
	.file	"partition_pure.c"
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
	.ident	"BiSheng Enterprise 4.2.0.2.B002 clang version 17.0.6 (2261d9fde4e0)"
	.section	".note.GNU-stack","",@progbits
	.addrsig
