#ifndef _PMAP_H_
#define _PMAP_H_

#include <mmu.h>
#include <queue.h>
#include <types.h>

extern Pde *cur_pgdir;

// PTE Record stores the page table entries points to the page
struct Pterec
{
	// page table is not swappable
	// pterec_ref points to the kernel address of page table entry
	Pte *pterec_ref;
	u_int pterec_asid;
	u_long pterec_va;
	LIST_ENTRY(Pterec)
	pterec_link;
};

LIST_HEAD(Pterec_list, Pterec);

extern struct Pterec pterecs[];
extern struct Pterec_list pterec_free_list;

LIST_HEAD(Page_list, Page);
typedef LIST_ENTRY(Page) Page_LIST_entry_t;

struct Page
{
	Page_LIST_entry_t pp_link; /* free list link */
	u_short pp_ref;
};

#endif /* _PMAP_H_ */
