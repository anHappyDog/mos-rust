#include <lib.h>

int main()
{
	int a = 0;
	int id = 0;

	if ((id = fork()) == 0)
	{
		if ((id = fork()) == 0)
		{
			a += 3;

			debugf("\t\tthis is child2 :a:%d\n", a);
			return 0;
		}

		a += 2;

		debugf("\tthis is child :a:%d\n", a);
		return 0;
	}

	a++;

	debugf("this is father: a:%d\n", a);

	return 0;
}
