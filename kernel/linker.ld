OUTPUT_FORMAT("pei-x86-64")
ENTRY(KeEntry)
SECTIONS
{
  .text : { *(.text*) }
  .rodata : { *(.rodata*) }
  .data : { *(.data*) }
  .bss : { *(.bss*) }
}
