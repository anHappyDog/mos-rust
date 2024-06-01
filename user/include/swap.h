#ifndef _SWAP_H_
#define _SWAP_H_

#include <pmap.h>

#ifdef MOS_NSWAP
#define NSWAP MOS_NSWAP
#else
#define NSWAP 0
#endif

#define LOG2NSWAPBUF 4
#define NSWAPBUF (1 << LOG2NSWAPBUF)

#define LOG2NPTEREC 16
#define NPTEREC (1 << LOG2NPTEREC)

#define SWAP_FREE 0
#define SWAP_BUFFER 1
#define SWAP_DISK 2

#define PAGE_SWAP 0
#define PAGE_RESERVED 1
#define PAGE_UNSWAP 2

#define SWAP_DISKNO 2

struct Swap {
	struct Page swap_page;
	u_int swap_status;
	u_int swap_buf_id;
	LIST_ENTRY(Swap) swap_link;
};

LIST_HEAD(Swap_list, Swap);

extern struct Swap swaps[];
extern char swap_buf[];

static inline int swap2id(struct Swap *s) {
	return s - swaps;
}

// swap.c
void swap_init(void);
int swap_in(struct Swap *s, struct Page *p);
int swap_out(struct Swap *s, struct Page *p);
int swap_alloc(struct Swap **new);
int swap_page(int npage);
int swap_lookup(Pte *pte);

int pterec_insert(struct Page *p, u_int asid, u_long va, Pte *pte);
int pterec_remove(struct Page *p, u_int asid, u_long va, Pte *pte);

// ide.c
#define SECT_SIZE 512

void ide_read_page(u_int pageno, void *dstva);
void ide_write_page(u_int pageno, void *srcva);

#endif
