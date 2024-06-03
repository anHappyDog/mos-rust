#include <env.h>
#include <lib.h>
#include <mmu.h>
#define debug 0

static int pipe_close(struct Fd *);
static int pipe_read(struct Fd *fd, void *buf, u_int n, u_int offset);
static int pipe_stat(struct Fd *, struct Stat *);
static int pipe_write(struct Fd *fd, const void *buf, u_int n, u_int offset);

struct Dev devpipe = {
    .dev_id = 'p',
    .dev_name = "pipe",
    .dev_read = pipe_read,
    .dev_write = pipe_write,
    .dev_close = pipe_close,
    .dev_stat = pipe_stat,
};

#define PIPE_SIZE 32 
struct Pipe {
	u_int p_rpos;		 
	u_int p_wpos;		 
	u_char p_buf[PIPE_SIZE];
};


int pipe(int pfd[2]) {
	int r;
	void *va;
	struct Fd *fd0, *fd1;


	if ((r = fd_alloc(&fd0)) < 0 || (r = syscall_mem_alloc(0, fd0, PTE_D | PTE_LIBRARY)) < 0) {
		goto err;
	}

	if ((r = fd_alloc(&fd1)) < 0 || (r = syscall_mem_alloc(0, fd1, PTE_D | PTE_LIBRARY)) < 0) {
		goto err1;
	}


	va = fd2data(fd0);
	if ((r = syscall_mem_alloc(0, (void *)va, PTE_D | PTE_LIBRARY)) < 0) {
		goto err2;
	}
	if ((r = syscall_mem_map(0, (void *)va, 0, (void *)fd2data(fd1), PTE_D | PTE_LIBRARY)) <
	    0) {
		goto err3;
	}


	fd0->fd_dev_id = devpipe.dev_id;
	fd0->fd_omode = O_RDONLY;

	fd1->fd_dev_id = devpipe.dev_id;
	fd1->fd_omode = O_WRONLY;

	debugf("[%08x] pipecreate \n", env->env_id, vpt[VPN(va)]);


	pfd[0] = fd2num(fd0);
	pfd[1] = fd2num(fd1);
	return 0;

err3:
	syscall_mem_unmap(0, (void *)va);
err2:
	syscall_mem_unmap(0, fd1);
err1:
	syscall_mem_unmap(0, fd0);
err:
	return r;
}

static int _pipe_is_closed(struct Fd *fd, struct Pipe *p) {

	int fd_ref, pipe_ref, runs;

	do {
		runs = env->env_runs;
		fd_ref = pageref(fd);
		pipe_ref = pageref(p);
	} while (runs != env->env_runs);

	return fd_ref == pipe_ref;
}

static int pipe_read(struct Fd *fd, void *vbuf, u_int n, u_int offset) {
	int i;
	struct Pipe *p;
	char *rbuf;

	p = (struct Pipe *)fd2data(fd);
	rbuf = (char *)vbuf;
	for (i = 0; i < n; ++i) {
		while (p->p_rpos == p->p_wpos) {
			if (_pipe_is_closed(fd, p) || i > 0) {
				return i;
			}
			syscall_yield();
		}
		rbuf[i] = p->p_buf[p->p_rpos % PIPE_SIZE];
		p->p_rpos++;
	}
	return n;

}

static int pipe_write(struct Fd *fd, const void *vbuf, u_int n, u_int offset) {
	int i;
	struct Pipe *p;
	char *wbuf;


	p = (struct Pipe *)fd2data(fd);
	wbuf = (char *)vbuf;
	for (i = 0; i < n; ++i) {
		while (p->p_wpos - p->p_rpos == PIPE_SIZE) {
			if (_pipe_is_closed(fd, p)) {
				return i;
			}
			syscall_yield();
		}
		p->p_buf[p->p_wpos % PIPE_SIZE] = wbuf[i];
		p->p_wpos++;
	}


	return n;
}

int pipe_is_closed(int fdnum) {
	struct Fd *fd;
	struct Pipe *p;
	int r;
	if ((r = fd_lookup(fdnum, &fd)) < 0) {
		return r;
	}
	p = (struct Pipe *)fd2data(fd);
	return _pipe_is_closed(fd, p);
}

static int pipe_close(struct Fd *fd) {
	syscall_mem_unmap(0, fd);
	syscall_mem_unmap(0, (void *)fd2data(fd));
	return 0;
}

static int pipe_stat(struct Fd *fd, struct Stat *stat) {
	return 0;
}
