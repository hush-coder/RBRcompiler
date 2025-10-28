	.text
	.file	"partition_pure.c"
	.globl	partition                       // -- Begin function partition
	.p2align	2
	.type	partition,@function
partition:                              // @partition
	.cfi_startproc
// %bb.0:
                                        // kill: def $w2 killed $w2 def $x2
	ldr	w10, [x0, w2, sxtw #2]
	mov	x8, x0
	sxtw	x9, w2
	sub	w11, w1, #1
	cmp	w1, w2
	b.ge	.LBB0_6
// %bb.1:
	add	x12, x8, w1, sxtw #2
	sub	x13, x9, w1, sxtw
	b	.LBB0_3
.LBB0_2:                                //   in Loop: Header=BB0_3 Depth=1
	add	x12, x12, #4
	subs	x13, x13, #1
	b.eq	.LBB0_5
.LBB0_3:                                // =>This Inner Loop Header: Depth=1
	ldr	w14, [x12]
	cmp	w14, w10
	b.gt	.LBB0_2
// %bb.4:                               //   in Loop: Header=BB0_3 Depth=1
	add	w11, w11, #1
	sbfiz	x15, x11, #2, #32
	ldr	w16, [x8, x15]
	str	w14, [x8, x15]
	str	w16, [x12]
	b	.LBB0_2
.LBB0_5:
	ldr	w10, [x8, x9, lsl #2]
.LBB0_6:
	add	w0, w11, #1
	sbfiz	x11, x0, #2, #32
                                        // kill: def $w0 killed $w0 killed $x0
	ldr	w12, [x8, x11]
	str	w10, [x8, x11]
	str	w12, [x8, x9, lsl #2]
	ret
.Lfunc_end0:
	.size	partition, .Lfunc_end0-partition
	.cfi_endproc
                                        // -- End function
	.ident	"BiSheng Enterprise 4.2.0.2.B002 clang version 17.0.6 (2261d9fde4e0)"
	.section	".note.GNU-stack","",@progbits
	.addrsig
