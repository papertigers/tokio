#include <stdint.h>
#include <sys/sdt.h>
#include "usdt.h"

void taskspawn(uint64_t id, char *name) {
    DTRACE_PROBE2(tokio, taskspawn, id, name);
}
