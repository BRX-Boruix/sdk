/* 3P3-1 验收：三条构造（#include <stdio.h> + int main + printf）的最小 C 程序。
 *
 * **注意那个显式的 return 0**：本工具链是 `-ffreestanding`，而 C 标准里
 * 「从 main 返回等价于 return 0」属于 **hosted** 环境——freestanding 下不成立。
 * 实测：省掉它时退出码会是 main 里最后一次调用的返回值（printf 返回 19 = 输出字节数）。
 */
#include <stdio.h>
int main(void) {
    printf("hello, three lines\n");
    return 0;
}