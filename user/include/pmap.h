#ifndef _PMAP_H_
#define _PMAP_H_

#include <mmu.h>
#include <printk.h>
#include <queue.h>
#include <types.h>

typedef LIST_ENTRY(Page) Page_LIST_entry_t;

struct Page
{
	Page_LIST_entry_t pp_link;
	u_short pp_ref;
};

extern struct Page *pages;

#endif /* _PMAP_H_ */
