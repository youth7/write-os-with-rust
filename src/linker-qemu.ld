OUTPUT_ARCH(riscv)
ENTRY(_start)/*引用了上面汇编代码中定义的全局符号_start*/
BASE_ADDRESS = 0x80200000;
SECTIONS
{
    . = BASE_ADDRESS;
    skernel = .;

    stext = .;
    .text : {
        *(.text.entry)
        *(.text .text.*)
    }

    . = ALIGN(4K);
    etext = .;
    srodata = .;
    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }

    . = ALIGN(4K);
    erodata = .;
    sdata = .;
    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }

    . = ALIGN(4K);
    edata = .;
    .bss : {
        *(.bss.stack)/*注意这是栈空间，为何栈空间需要写入ELF呢？*/
        sbss = .;	/*sbss和ebss之间的内容需要初始化为0,但是.bss.stack不需要初始化，因此.bss.stack不在sbss和ebss之间*/
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    }

    . = ALIGN(4K);
    ebss = .;
    ekernel = .;

    /DISCARD/ : {
        *(.eh_frame)
    }
}