#ifndef _PMAP_H_
#define _PMAP_H_

#include <mmu.h>
#include <queue.h>
#include <types.h>


typedef LIST_ENTRY(Page) Page_LIST_entry_t;

struct Page
{
	Page_LIST_entry_t pp_link; /* free list link */
	u_short pp_ref;
};

#endif /* _PMAP_H_ */
