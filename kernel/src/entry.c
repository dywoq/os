/**
 * A OS entry.
 *
 * WARNING: DO NOT CHANGE THE FUNCTION NAME,
 * IT IS HARDCODED INTO LINKER SCRIPT
 */
void
KeEntry()
{
    while (1)
    {
        asm volatile("hlt");
    }
}
