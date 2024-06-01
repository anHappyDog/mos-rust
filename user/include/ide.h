#ifndef _SWAP_H_
#define _SWAP_H_

void ide_read_page(u_int pageno, void *dstva);
void ide_write_page(u_int pageno, void *srcva);

#endif
