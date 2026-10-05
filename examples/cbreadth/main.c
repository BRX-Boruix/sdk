/* cbreadth - BORUIX C 体验的覆盖面驱动（3P3-2）。
 *
 * 目的：用**一个真实的 C 程序**同时压过 ctype / string / stdlib / math / stdio / assert /
 * setjmp / atexit / mmap 八族，证明「基础 libc 缺失」不再卡住 C 移植。每条检查都打印结论，
 * 失败计数决定退出码。
 *
 * 注意：本系统没有单独的 libm——math 符号就在 libc.a 内，故不需要 -lm。
 */
#include <assert.h>
#include <ctype.h>
#include <math.h>
#include <setjmp.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/mman.h>

static int failures = 0;
static volatile int atexit_ran = 0;
static volatile int sig_seen = 0;

static void check(int ok, const char *what) {
    if (!ok) {
        printf("FAIL: %s\n", what);
        failures++;
    }
}

static void on_exit_handler(void) {
    atexit_ran++;
    /* 打印出来，使「处理函数确实在 exit 时运行过」在日志里可见（而非只靠退出码）。 */
    printf("atexit: handler ran (count=%d)\n", atexit_ran);
}
static void on_signal(int sig) { sig_seen = sig; }

int main(void) {
    /* ctype */
    check(isalpha('A') && !isalpha('1'), "isalpha");
    check(isdigit('7') && !isdigit('x'), "isdigit");
    check(isspace(' ') && isspace('\n'), "isspace");
    check(toupper('a') == 'A' && tolower('Z') == 'z', "case conversion");
    check(isxdigit('f') && ispunct(',') && isprint('~') && !isprint(0x1f), "class predicates");

    /* string + stdlib */
    char buf[64];
    strcpy(buf, "Hello, Boruix");
    check(strlen(buf) == 13, "strlen");
    check(strcmp(buf, "Hello, Boruix") == 0, "strcmp");
    check(strchr(buf, ',') != NULL, "strchr");
    check(atoi("-42") == -42, "atoi");
    check((int)strtol("0x1f", NULL, 0) == 31, "strtol base 0");

    /* math（符号在 libc.a 内，无需 -lm） */
    check(sqrt(16.0) == 4.0, "sqrt");
    check(floor(3.7) == 3.0 && ceil(3.2) == 4.0, "floor/ceil");
    check(fabs(-2.5) == 2.5, "fabs");
    check(pow(2.0, 10.0) == 1024.0, "pow");

    /* stdio 格式化（含变长浮点） */
    char out[32];
    snprintf(out, sizeof out, "%d/%s/%.1f", 7, "ok", 1.5);
    check(strcmp(out, "7/ok/1.5") == 0, "snprintf");

    /* setjmp/longjmp：非局部跳转（jumped 必须 volatile，否则跨越 longjmp 后取值不定） */
    jmp_buf jb;
    volatile int jumped = 0;
    if (setjmp(jb) == 0) {
        jumped = 1;
        longjmp(jb, 7);
        check(0, "longjmp did not transfer control");
    } else {
        check(jumped == 1, "setjmp/longjmp round trip");
    }

    /* mmap/munmap */
    void *p = mmap(NULL, 4096, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    check(p != MAP_FAILED, "mmap");
    if (p != MAP_FAILED) {
        ((char *)p)[0] = 'x';
        check(munmap(p, 4096) == 0, "munmap");
    }

    /* signal：装 handler 并 raise 它 */
    check(signal(SIGUSR1, on_signal) != SIG_ERR, "signal");
    check(raise(SIGUSR1) == 0, "raise");
    check(sig_seen == SIGUSR1, "handler saw SIGUSR1");

    /* assert 宏本身：成功路径不应触发 */
    assert(strlen(buf) == 13);

    /* atexit：登记的处理函数应在 exit 时运行（本函数随后用 exit 返回，故它会打印） */
    check(atexit(on_exit_handler) == 0, "atexit registration");

    printf("cbreadth: %d checks failed\n", failures);
    if (atexit_ran != 0) {
        printf("FAIL: atexit handler ran before exit\n");
        failures++;
    }
    exit(failures == 0 ? 0 : 1);
}