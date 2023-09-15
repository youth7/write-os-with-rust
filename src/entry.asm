.section .text.entry	#定义名为.text.entry的section，它稍后会被链接脚本引用
.globl _start	#定义全局符号_start，使其对外可见，它是一个地址，稍后会被链接脚本引用
_start:
    la sp, boot_stack_top	#设置栈顶值，boot_stack_top此时是指向栈顶的
    call rust_main			#上面设置好sp之后，才能正常调用rust程序
    .section .bss.stack #定义了一个section，后续的内容都会进入这个section，包括后面.space中指定的，注意.bss.stack已经包含在链接脚本中了
    .globl boot_stack_lower_bound	#定义全局符号
boot_stack_lower_bound:	#栈底 
    .space 4096 * 16	#分配栈空间
    .globl boot_stack_top	#定义全局符号
boot_stack_top:			#栈顶