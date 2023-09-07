.section .text.entry	#.text.entry是一个地址，稍后会被链接脚本引用
.globl _start	#_start是一个地址，稍后会被链接脚本引用
 _start:
     li x1, 100