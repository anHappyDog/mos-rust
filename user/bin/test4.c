#include <lib.h>

const int addr[] = {
    0x6fffe000, 0x6ffff000, 0x70000000};
int main(int argc, char **argv)
{

    int fd = open("/motd", O_RDONLY);
    if (fd < 0) {
        user_panic("open /motd: %d", fd);
    }
    char buf[512 + 1];
    int n;
    while ((n = read(fd, buf, sizeof buf - 1)) > 0) {
        buf[n] = 0;
        debugf("%s\n", buf);
    }
    close(fd);

    return 0;
}