/**
 * A OS entry.
 *
 * WARNING: DON'T CHANGE THE FUNCTION NAME,
 * IT'S HARDCODED INTO LINKER SCRIPT
 */
void
KeEntry()
{
    while (1)
    {
        asm volatile("hlt");
    }
}
