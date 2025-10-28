	.text
	.file	"test_partition.c"
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
	.globl	generate_array                  // -- Begin function generate_array
	.p2align	2
	.type	generate_array,@function
generate_array:                         // @generate_array
	.cfi_startproc
// %bb.0:
	stp	x29, x30, [sp, #-48]!           // 16-byte Folded Spill
	.cfi_def_cfa_offset 48
	stp	x22, x21, [sp, #16]             // 16-byte Folded Spill
	stp	x20, x19, [sp, #32]             // 16-byte Folded Spill
	mov	x29, sp
	.cfi_def_cfa w29, 48
	.cfi_offset w19, -8
	.cfi_offset w20, -16
	.cfi_offset w21, -24
	.cfi_offset w22, -32
	.cfi_offset w30, -40
	.cfi_offset w29, -48
	mov	x19, x0
	mov	w0, #42                         // =0x2a
	mov	w20, w1
	bl	srand
	cmp	w20, #1
	b.lt	.LBB1_3
// %bb.1:
	mov	w21, #35757                     // =0x8bad
	mov	w20, w20
	movk	w21, #26843, lsl #16
	mov	w22, #10000                     // =0x2710
.LBB1_2:                                // =>This Inner Loop Header: Depth=1
	bl	rand
	smull	x8, w0, w21
	subs	x20, x20, #1
	lsr	x9, x8, #63
	asr	x8, x8, #44
	add	w8, w8, w9
	msub	w8, w8, w22, w0
	str	w8, [x19], #4
	b.ne	.LBB1_2
.LBB1_3:
	.cfi_def_cfa wsp, 48
	ldp	x20, x19, [sp, #32]             // 16-byte Folded Reload
	ldp	x22, x21, [sp, #16]             // 16-byte Folded Reload
	ldp	x29, x30, [sp], #48             // 16-byte Folded Reload
	.cfi_def_cfa_offset 0
	.cfi_restore w19
	.cfi_restore w20
	.cfi_restore w21
	.cfi_restore w22
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
	stp	x29, x30, [sp, #-96]!           // 16-byte Folded Spill
	.cfi_def_cfa_offset 96
	stp	x28, x27, [sp, #16]             // 16-byte Folded Spill
	stp	x26, x25, [sp, #32]             // 16-byte Folded Spill
	stp	x24, x23, [sp, #48]             // 16-byte Folded Spill
	stp	x22, x21, [sp, #64]             // 16-byte Folded Spill
	stp	x20, x19, [sp, #80]             // 16-byte Folded Spill
	mov	x29, sp
	.cfi_def_cfa w29, 96
	.cfi_offset w19, -8
	.cfi_offset w20, -16
	.cfi_offset w21, -24
	.cfi_offset w22, -32
	.cfi_offset w23, -40
	.cfi_offset w24, -48
	.cfi_offset w25, -56
	.cfi_offset w26, -64
	.cfi_offset w27, -72
	.cfi_offset w28, -80
	.cfi_offset w30, -88
	.cfi_offset w29, -96
	sub	sp, sp, #48, lsl #12            // =196608
	sub	sp, sp, #3392
	mov	w0, #42                         // =0x2a
	bl	srand
	mov	w21, #35757                     // =0x8bad
	mov	x19, sp
	mov	w20, #50000                     // =0xc350
	movk	w21, #26843, lsl #16
	mov	w22, #10000                     // =0x2710
.LBB2_1:                                // =>This Inner Loop Header: Depth=1
	bl	rand
	smull	x8, w0, w21
	subs	x20, x20, #1
	lsr	x9, x8, #63
	asr	x8, x8, #44
	add	w8, w8, w9
	msub	w8, w8, w22, w0
	str	w8, [x19], #4
	b.ne	.LBB2_1
// %bb.2:
	mov	x23, sp
	mov	w25, #35757                     // =0x8bad
	add	x8, x23, #48, lsl #12           // =196608
	mov	x21, xzr
	mov	w22, wzr
	add	x24, x8, #3388
	movk	w25, #26843, lsl #16
	mov	w26, #10000                     // =0x2710
	b	.LBB2_4
.LBB2_3:                                //   in Loop: Header=BB2_4 Depth=1
	add	w27, w8, #1
	ldr	w10, [x24]
	sbfiz	x8, x27, #2, #32
	ldr	w9, [x23, x8]
	str	w10, [x23, x8]
	str	w9, [x24]
	bl	clock
	sub	x8, x21, x19
	add	w20, w27, w20
	add	x21, x8, x0
	add	w22, w22, #1
	cmp	w22, #100
	b.eq	.LBB2_10
.LBB2_4:                                // =>This Loop Header: Depth=1
                                        //     Child Loop BB2_5 Depth 2
                                        //     Child Loop BB2_8 Depth 2
	mov	w0, #42                         // =0x2a
	bl	srand
	mov	x19, sp
	mov	w27, #50000                     // =0xc350
.LBB2_5:                                //   Parent Loop BB2_4 Depth=1
                                        // =>  This Inner Loop Header: Depth=2
	bl	rand
	smull	x8, w0, w25
	subs	x27, x27, #1
	lsr	x9, x8, #63
	asr	x8, x8, #44
	add	w8, w8, w9
	msub	w8, w8, w26, w0
	str	w8, [x19], #4
	b.ne	.LBB2_5
// %bb.6:                               //   in Loop: Header=BB2_4 Depth=1
	bl	clock
	mov	x19, x0
	ldr	w9, [x24]
	mov	w8, #-1                         // =0xffffffff
	mov	x10, sp
	mov	w11, #49999                     // =0xc34f
	b	.LBB2_8
.LBB2_7:                                //   in Loop: Header=BB2_8 Depth=2
	subs	x11, x11, #1
	add	x10, x10, #4
	b.eq	.LBB2_3
.LBB2_8:                                //   Parent Loop BB2_4 Depth=1
                                        // =>  This Inner Loop Header: Depth=2
	ldr	w12, [x10]
	cmp	w12, w9
	b.gt	.LBB2_7
// %bb.9:                               //   in Loop: Header=BB2_8 Depth=2
	add	w8, w8, #1
	sbfiz	x13, x8, #2, #32
	ldr	w14, [x23, x13]
	str	w12, [x23, x13]
	str	w14, [x10]
	b	.LBB2_7
.LBB2_10:
	mov	x8, #145685290680320            // =0x848000000000
	scvtf	d0, x21
	movk	x8, #16686, lsl #48
	adrp	x0, .L.str
	add	x0, x0, :lo12:.L.str
	fmov	d1, x8
	mov	x8, #70368744177664             // =0x400000000000
	movk	x8, #16527, lsl #48
	fdiv	d0, d0, d1
	fmov	d1, x8
	mov	x8, #4636737291354636288        // =0x4059000000000000
	fmul	d0, d0, d1
	fmov	d1, x8
	fdiv	d0, d0, d1
	bl	printf
	adrp	x0, .Lstr
	add	x0, x0, :lo12:.Lstr
	bl	puts
	negs	w8, w20
	and	w9, w20, #0xff
	and	w8, w8, #0xff
	csneg	w0, w9, w8, mi
	add	sp, sp, #48, lsl #12            // =196608
	add	sp, sp, #3392
	.cfi_def_cfa wsp, 96
	ldp	x20, x19, [sp, #80]             // 16-byte Folded Reload
	ldp	x22, x21, [sp, #64]             // 16-byte Folded Reload
	ldp	x24, x23, [sp, #48]             // 16-byte Folded Reload
	ldp	x26, x25, [sp, #32]             // 16-byte Folded Reload
	ldp	x28, x27, [sp, #16]             // 16-byte Folded Reload
	ldp	x29, x30, [sp], #96             // 16-byte Folded Reload
	.cfi_def_cfa_offset 0
	.cfi_restore w19
	.cfi_restore w20
	.cfi_restore w21
	.cfi_restore w22
	.cfi_restore w23
	.cfi_restore w24
	.cfi_restore w25
	.cfi_restore w26
	.cfi_restore w27
	.cfi_restore w28
	.cfi_restore w30
	.cfi_restore w29
	ret
.Lfunc_end2:
	.size	main, .Lfunc_end2-main
	.cfi_endproc
                                        // -- End function
	.type	.L.str,@object                  // @.str
	.section	.rodata.str1.1,"aMS",@progbits,1
.L.str:
	.asciz	"\345\210\206\345\214\272\345\207\275\346\225\260\345\271\263\345\235\207\346\211\247\350\241\214\346\227\266\351\227\264: %.3f \346\257\253\347\247\222\n"
	.size	.L.str, 45

	.type	.Lstr,@object                   // @str
.Lstr:
	.asciz	"\346\200\273\345\205\261\346\211\247\350\241\214: 100 \346\254\241"
	.size	.Lstr, 22

	.ident	"BiSheng Enterprise 4.2.0.2.B002 clang version 17.0.6 (2261d9fde4e0)"
	.section	".note.GNU-stack","",@progbits
	.addrsig
