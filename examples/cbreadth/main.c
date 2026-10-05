/* cbreadth - BORUIX C 体验的覆盖面驱动（3P3-2）。
 *
 * 目的：用**一个真实的 C 程序**同时压过 ctype / string / stdlib / math / stdio / assert 六族，
 * 证明「基础 libc 缺失」不再卡住 C 移植。每条检查都打印结论，失败计数决定退出码。
 *
 * 注意：本系统没有单独的 libm——math 符号就在 libc.a 内，故不需要 -lm。
 */
#include <assert.h>
#include <ctype.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static int failures = 0;

static void check(int ok, const char *what) {
    if (!ok) {
        printf("FAIL: %s\n", what);
        failures++;
    }
}

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

    /* stdio 格式化 */
    char out[32];
    snprintf(out, sizeof out, "%d/%s/%.1f", 7, "ok", 1.5);
    check(strcmp(out, "7/ok/1.5") == 0, "snprintf");

    /* assert 宏本身：成功路径不应触发 */
    assert(strlen(buf) == 13);

    printf("cbreadth: %d checks failed\n", failures);
    return failures == 0 ? 0 : 1;
}