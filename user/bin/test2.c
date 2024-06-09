#include <lib.h>

int main(int argc, char **argv)
{
    const u_int addr[] = {0x6FFE0000, 0x6FFE1000};
    int r = syscall_mem_alloc(0, addr[0], PTE_LIBRARY | PTE_D);
    if (r < 0)
    {
        user_panic("SYS_mem_alloc returned %d", r);
    }
    char *msg = "Now is the time for all good";
    char *msg2 = "Something is not good ,while others are not.";
    memcpy(addr[0], msg, strlen(msg));
    r = syscall_mem_map(0, addr[0], 0, addr[1], PTE_LIBRARY | PTE_D);
    if (r < 0)
    {
        user_panic("SYS_mem_map returned %d", r);
    }
    int pid = fork();
    if (pid < 0)
    {
        user_panic("fork: %d", pid);
    }
    if (pid == 0)
    {
        if (strcmp(addr[0], addr[1]) == 0)
        {
            debugf("\nmem map in fork properly\n");
        }
        else
        {
            debugf("\nmem map in fork failed\n");
        }
        memcpy(addr[0], msg2, strlen(msg2));
        debugf("\n the str is %s\n", addr[0]);
        return 0;
    }
    else
    {
        wait(pid);

        if (strcmp(addr[0], addr[1]) == 0)
        {
            debugf("\nmem map in parent properly\n");
        }
        if (strcmp(addr[0],msg2) == 0) {
            debugf("\nmem map in parent properly\n");
        }
        else
        {
            debugf("\nmem map in parent failed\n");
        }
    }

    r = syscall_mem_unmap(0, addr[1]);
    if (r < 0)
    {
        user_panic("SYS_mem_unmap returned %d", r);
    }
    r = syscall_mem_unmap(0, addr[0]);
    if (r < 0)
    {
        user_panic("SYS_mem_unmap returned %d", r);
    }
    debugf("\nmem unmap properly\n");
    return 0;
}