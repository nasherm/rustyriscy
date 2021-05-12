# RISC-V assembly
The ISA can be obtained at this link [RISC-V spec](https://riscv.org/technical/specifications/)

## Registers
RISC-V defines various registers. There are general purpose, control and registers that come with extensions such as
floating point for F extensions and vectors for the V extension.

With RV32I (RISC-V 32-bit ISA) there 32 registers x0-x31, not including the PC register. The x0 register is read only,
only ever returning 0. Some registers have a specific purpose. The following table lists the differences


Register  | ABI         | Use by convention                     | Preserved?
:-------- | :---------- | :---------------                      | ------
x0        | zero        | hardwired to 0, ignores writes        | _n/a_
x1        | ra          | return address for jumps              | no
x2        | sp          | stack pointer                         | yes
x3        | gp          | global pointer                        | _n/a_
x4        | tp          | thread pointer                        | _n/a_
x5        | t0          | temporary register 0                  | no
x6        | t1          | temporary register 1                  | no
x7        | t2          | temporary register 2                  | no
x8        | s0 _or_ fp  | saved register 0 _or_ frame pointer   | yes
x9        | s1          | saved register 1                      | yes
x10       | a0          | return value _or_ function argument 0 | no
x11       | a1          | return value _or_ function argument 1 | no
x12       | a2          | function argument 2                   | no
x13       | a3          | function argument 3                   | no
x14       | a4          | function argument 4                   | no
x15       | a5          | function argument 5                   | no
x16       | a6          | function argument 6                   | no
x17       | a7          | function argument 7                   | no
x18       | s2          | saved register 2                      | yes
x19       | s3          | saved register 3                      | yes
x20       | s4          | saved register 4                      | yes
x21       | s5          | saved register 5                      | yes
x22       | s6          | saved register 6                      | yes
x23       | s7          | saved register 7                      | yes
x24       | s8          | saved register 8                      | yes
x25       | s9          | saved register 9                      | yes
x26       | s10         | saved register 10                     | yes
x27       | s11         | saved register 11                     | yes
x28       | t3          | temporary register 3                  | no
x29       | t4          | temporary register 4                  | no
x30       | t5          | temporary register 5                  | no
x31       | t6          | temporary register 6                  | no
pc        | _(none)_    | program counter                       | _n/a_

By convention, saved registers are preserved across function call, and the argument registers a0 to a7 and
temporary registers t0 to t6 aren't.

## Addressing
Addressing is of the format `%pcrel_lo()`

## Assembler directives
The assembler provides the following directives

Directive    | Arguments                      | Description
:----------- | :-------------                 | :---------------
.align       | integer                        | align to power of 2 (alias for .p2align)
.file        | "filename"                     | emit filename FILE LOCAL symbol table
.globl       | symbol_name                    | emit symbol_name to symbol table (scope GLOBAL)
.local       | symbol_name                    | emit symbol_name to symbol table (scope LOCAL)
.comm        | symbol_name,size,align         | emit common object to .bss section
.common      | symbol_name,size,align         | emit common object to .bss section
.ident       | "string"                       | accepted for source compatibility
.section     | [{.text,.data,.rodata,.bss}]   | emit section (if not present, default .text) and make current
.size        | symbol, symbol                 | accepted for source compatibility
.text        |                                | emit .text section (if not present) and make current
.data        |                                | emit .data section (if not present) and make current
.rodata      |                                | emit .rodata section (if not present) and make current
.bss         |                                | emit .bss section (if not present) and make current
.string      | "string"                       | emit string
.asciz       | "string"                       | emit string (alias for .string)
.equ         | name, value                    | constant definition
.macro       | name arg1 [, argn]             | begin macro definition \argname to substitute
.endm        |                                | end macro definition
.type        | symbol, @function              | accepted for source compatibility
.option      | {rvc,norvc,pic,nopic,push,pop} | RISC-V options
.byte        | expression [, expression]*     | 8-bit comma separated words
.2byte       | expression [, expression]*     | 16-bit comma separated words
.half        | expression [, expression]*     | 16-bit comma separated words
.short       | expression [, expression]*     | 16-bit comma separated words
.4byte       | expression [, expression]*     | 32-bit comma separated words
.word        | expression [, expression]*     | 32-bit comma separated words
.long        | expression [, expression]*     | 32-bit comma separated words
.8byte       | expression [, expression]*     | 64-bit comma separated words
.dword       | expression [, expression]*     | 64-bit comma separated words
.quad        | expression [, expression]*     | 64-bit comma separated words
.dtprelword  | expression [, expression]*     | 32-bit thread local word
.dtpreldword | expression [, expression]*     | 64-bit thread local word
.sleb128     | expression                     | signed little endian base 128, DWARF
.uleb128     | expression                     | unsigned little endian base 128, DWARF
.p2align     | p2,[pad_val=0],max             | align to power of 2
.balign      | b,[pad_val=0]                  | byte align
.zero        | integer                        | zero bytes

## Relocation functions

Assembler Notation          | Description                    | Instruction / Macro
:----------------------     | :---------------               | :-------------------
%hi(symbol)                 | Absolute (HI20)                | lui
%lo(symbol)                 | Absolute (LO12)                | load, store, add
%pcrel_hi(symbol)           | PC-relative (HI20)             | auipc
%pcrel_lo(label)            | PC-relative (LO12)             | load, store, add
%tprel_hi(symbol)           | TLS LE "Local Exec"            | lui
%tprel_lo(symbol)           | TLS LE "Local Exec"            | load, store, add
%tprel_add(symbol)          | TLS LE "Local Exec"            | add
%tls_ie_pcrel_hi(symbol) \* | TLS IE "Initial Exec" (HI20)   | auipc
%tls_gd_pcrel_hi(symbol) \* | TLS GD "Global Dynamic" (HI20) | auipc
%got_pcrel_hi(symbol) \*    | GOT PC-relative (HI20)


## Labels

Labels can take two forms. We can have text labels such as:

```assembly
my_function:
    ....
    j my_function
```

and we can have numeric labels. With these we need to specify whether we have a forward reference or backward
reference by suffixing 'f' or 'b' respectively.

```assembly
1:
    j 2f
2:
    j 1b
3:
    j 3b
```

## Addressing
### Absolute
```assembly
    lui a0, %hi(msg + 1)
    addi a0, a0, %lo(msg + 1)
```

### Relative to PC
```assembly
1:
    auipc a0, %pcrel_hi(msg + 1)
    addi  a0, a0, %pcrel_lo(1b)
```

### From the global offset table (GOT indirect)
```assembly
auipc a0, %got_pcrel_hi(msg + 1)
ld    a0, %pcrel_lo(1b)(a0)
```

## Load
`li` is used to load immediate instructions.

```assembly
.equ CONSTANT, 0xdeadbeef
li a0, CONSTANT
```

`la` is used to load addresses

```ass
la a0, msg + 1
```

To load and store from global objects we have the following
The following pseudo instructions are available to load from and store to
global objects:

  * `l{b|h|w|d} <rd>, <symbol>`: load byte, half word, word or double word from global[^1]
  * `s{b|h|w|d} <rd>, <symbol>, <rt>`: store byte, half word, word or double word to global[^2]
  * `fl{h|w|d|q} <rd>, <symbol>, <rt>`: load half, float, double or quad precision from global[^2]
  * `fs{h|w|d|q} <rd>, <symbol>, <rt>`: store half, float, double or quad precision to global[^2]

[^1]: the first operand is implicitly used as a scratch register.
[^2]: the last operand specifies the scratch register to be used.

## Function calls
The following instructions are available for function calls

  * `call	<symbol>`: call away subroutine[^1]
  * `call	<rd>, <symbol>`: call away subroutine[^2]
  * `tail	<symbol>`: tail call away subroutine[^3]
  * `jump	<symbol>, <rt>`: jump to away routine[^4]

[^1]: `ra` is implicitly used to save the return address.
[^2]: similar to `call <symbol>`, but `<rd>` is used to save the return address instead.
[^3]: `t1` is implicitly used as a scratch register.
[^4]: similar to `tail <symbol>`, but `<rt>` is used as the scratch register instead.

## Contraol and status registers
To illustrate control and status registers, we have the following code which does a set and
wait for a timer interrupt:

```assembly
.equ RTC_BASE,      0x40000000
.equ TIMER_BASE,    0x40004000

# setup machine trap vector
1:      auipc   t0, %pcrel_hi(mtvec)        # load mtvec(hi)
        addi    t0, t0, %pcrel_lo(1b)       # load mtvec(lo)
        csrrw   zero, mtvec, t0

# set mstatus.MIE=1 (enable M mode interrupt)
        li      t0, 8
        csrrs   zero, mstatus, t0

# set mie.MTIE=1 (enable M mode timer interrupts)
        li      t0, 128
        csrrs   zero, mie, t0

# read from mtime
        li      a0, RTC_BASE
        ld      a1, 0(a0)

# write to mtimecmp
        li      a0, TIMER_BASE
        li      t0, 1000000000
        add     a1, a1, t0
        sd      a1, 0(a0)

# loop
loop:
        wfi
        j loop

# break on interrupt
mtvec:
        csrrc  t0, mcause, zero
        bgez t0, fail       # interrupt causes are less than zero
        slli t0, t0, 1      # shift off high bit
        srli t0, t0, 1
        li t1, 7            # check this is an m_timer interrupt
        bne t0, t1, fail
        j pass

pass:
        la a0, pass_msg
        jal puts
        j shutdown

fail:
        la a0, fail_msg
        jal puts
        j shutdown

.section .rodata

pass_msg:
        .string "PASS\n"

fail_msg:
        .string "FAIL\n"
```


## A listing of standard RISC-V pseudoinstructions

Pseudoinstruction            | Base Instruction(s)                                           | Meaning   | Comment
:----------------------------|:--------------------------------------------------------------|:----------|:--------|
la rd, symbol                | auipc rd, symbol[31:12]; addi rd, rd, symbol[11:0]            | Load address
l{b\|h\|w\|d} rd, symbol     | auipc rd, symbol[31:12]; l{b\|h\|w\|d} rd, symbol[11:0]\(rd\) | Load global
s{b\|h\|w\|d} rd, symbol, rt | auipc rt, symbol[31:12]; s{b\|h\|w\|d} rd, symbol[11:0]\(rt\) | Store global
fl{w\|d} rd, symbol, rt      | auipc rt, symbol[31:12]; fl{w\|d} rd, symbol[11:0]\(rt\)      | Floating-point load global
fs{w\|d} rd, symbol, rt      | auipc rt, symbol[31:12]; fs{w\|d} rd, symbol[11:0]\(rt\)      | Floating-point store global
nop                          | addi x0, x0, 0                                                | No operation
li rd, immediate             | *Myriad sequences*                                            | Load immediate
mv rd, rs                    | addi rd, rs, 0                                                | Copy register
not rd, rs                   | xori rd, rs, -1                                               | One’s complement
neg rd, rs                   | sub rd, x0, rs                                                | Two’s complement
negw rd, rs                  | subw rd, x0, rs                                               | Two’s complement word
sext.b rd, rs                | slli rd, rs, XLEN - 8; srai rd, rd, XLEN - 8                  | Sign extend byte | It will expand to another instruction sequence when B extension is available*[1]
sext.h rd, rs                | slli rd, rs, XLEN - 16; srai rd, rd, XLEN - 16                | Sign extend half word | It will expand to another instruction sequence when B extension is available*[1]
sext.w rd, rs                | addiw rd, rs, 0                                               | Sign extend word
zext.b rd, rs                | andi rd, rs, 255                                              | Zero extend byte
zext.h rd, rs                | slli rd, rs, XLEN - 16; srli rd, rd, XLEN - 16                | Zero extend half word | It will expand to another instruction sequence when B extension is available*[1]
zext.w rd, rs                | slli rd, rs, XLEN - 32; srli rd, rd, XLEN - 32                | Zero extend word | It will expand to another instruction sequence when B extension is available*[1]
seqz rd, rs                  | sltiu rd, rs, 1                                               | Set if = zero
snez rd, rs                  | sltu rd, x0, rs                                               | Set if != zero
sltz rd, rs                  | slt rd, rs, x0                                                | Set if < zero
sgtz rd, rs                  | slt rd, x0, rs                                                | Set if > zero
fmv.s rd, rs                 | fsgnj.s rd, rs, rs                                            | Copy single-precision register
fabs.s rd, rs                | fsgnjx.s rd, rs, rs                                           | Single-precision absolute value
fneg.s rd, rs                | fsgnjn.s rd, rs, rs                                           | Single-precision negate
fmv.d rd, rs                 | fsgnj.d rd, rs, rs                                            | Copy double-precision register
fabs.d rd, rs                | fsgnjx.d rd, rs, rs                                           | Double-precision absolute value
fneg.d rd, rs                | fsgnjn.d rd, rs, rs                                           | Double-precision negate
beqz rs, offset              | beq rs, x0, offset                                            | Branch if = zero
bnez rs, offset              | bne rs, x0, offset                                            | Branch if != zero
blez rs, offset              | bge x0, rs, offset                                            | Branch if ≤ zero
bgez rs, offset              | bge rs, x0, offset                                            | Branch if ≥ zero
bltz rs, offset              | blt rs, x0, offset                                            | Branch if < zero
bgtz rs, offset              | blt x0, rs, offset                                            | Branch if > zero
bgt rs, rt, offset           | blt rt, rs, offset                                            | Branch if >
ble rs, rt, offset           | bge rt, rs, offset                                            | Branch if ≤
bgtu rs, rt, offset          | bltu rt, rs, offset                                           | Branch if >, unsigned
bleu rs, rt, offset          | bgeu rt, rs, offset                                           | Branch if ≤, unsigned
j offset                     | jal x0, offset                                                | Jump
jal offset                   | jal x1, offset                                                | Jump and link
jr rs                        | jalr x0, rs, 0                                                | Jump register
jalr rs                      | jalr x1, rs, 0                                                | Jump and link register
ret                          | jalr x0, x1, 0                                                | Return from subroutine
call offset                  | auipc x6, offset[31:12]; jalr x1, x6, offset[11:0]            | Call far-away subroutine
tail offset                  | auipc x6, offset[31:12]; jalr x0, x6, offset[11:0]            | Tail call far-away subroutine
fence                        | fence iorw, iorw                                              | Fence on all memory and I/O

* [1] We don't specify the code sequence when the B-extension is present, since B-extension still not ratified or frozen. We will specify the expansion sequence once it's frozen.

## Pseudoinstructions for accessing control and status registers

Pseudoinstruction | Base Instruction(s)        | Meaning
:---------------- |:---------------------------|:-------
rdinstret[h] rd   | csrrs rd, instret[h], x0   | Read instructions-retired counter
rdcycle[h] rd     | csrrs rd, cycle[h], x0     | Read cycle counter
rdtime[h] rd      | csrrs rd, time[h], x0      | Read real-time clock
csrr rd, csr      | csrrs rd, csr, x0          | Read CSR
csrw csr, rs      | csrrw x0, csr, rs          | Write CSR
csrs csr, rs      | csrrs x0, csr, rs          | Set bits in CSR
csrc csr, rs      | csrrc x0, csr, rs          | Clear bits in CSR
csrwi csr, imm    | csrrwi x0, csr, imm        | Write CSR, immediate
csrsi csr, imm    | csrrsi x0, csr, imm        | Set bits in CSR, immediate
csrci csr, imm    | csrrci x0, csr, imm        | Clear bits in CSR, immediate
frcsr rd          | csrrs rd, fcsr, x0         | Read FP control/status register
fscsr rd, rs      | csrrw rd, fcsr, rs         | Swap FP control/status register
fscsr rs          | csrrw x0, fcsr, rs         | Write FP control/status register
frrm rd           | csrrs rd, frm, x0          | Read FP rounding mode
fsrm rd, rs       | csrrw rd, frm, rs          | Swap FP rounding mode
fsrm rs           | csrrw x0, frm, rs          | Write FP rounding mode
fsrmi rd, imm     | csrrwi rd, frm, imm        | Swap FP rounding mode, immediate
fsrmi imm         | csrrwi x0, frm, imm        | Write FP rounding mode, immediate
frflags rd        | csrrs rd, fflags, x0       | Read FP exception flags
fsflags rd, rs    | csrrw rd, fflags, rs       | Swap FP exception flags
fsflags rs        | csrrw x0, fflags, rs       | Write FP exception flags
fsflagsi rd, imm  | csrrwi rd, fflags, imm     | Swap FP exception flags, immediate
fsflagsi imm      | csrrwi x0, fflags, imm     | Write FP exception flags, immediate
