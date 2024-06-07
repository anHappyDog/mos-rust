#include <lib.h>

int main(int argc, char **argv)
{
    for (int i = 0; i < 10; ++i)
    {
        debugf("env i is %d\n", envs[i].env_id);
    }
    return 0;
}