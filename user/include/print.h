#ifndef _print_h_
#define _print_h_

#include <stdarg.h>
#include <types.h>

typedef void (*fmt_callback_t)(void *data, const char *buf, size_t len);
void vprintfmt(fmt_callback_t out, void *data, const char *fmt, va_list ap);

#endif
