#ifndef CLIB_H
#define CLIB_H

#include <stddef.h>

/* Case 1: struct passed by pointer (repr(C) layout compatibility). */
typedef struct {
    double x;
    double y;
} Point;

double point_norm(const Point *p);

/* Case 2: pointer + length, the classic C array idiom. */
double sum_array(const double *data, size_t len);

/* Case 3: opaque handle. Layout is private to clib.c; Rust only ever holds
 * a pointer, never inspects the fields. Ownership (malloc/free) crosses the
 * FFI boundary and gets wrapped in a Rust Drop impl on the other side. */
typedef struct Counter Counter;

Counter *counter_new(int start);
void counter_inc(Counter *c);
int counter_get(const Counter *c);
void counter_free(Counter *c);

/* Case 4 (bonus): C calls back into Rust through a function pointer, plus an
 * opaque user_data pointer so the callback can carry state (a closure, on
 * the Rust side). */
typedef void (*ForEachCallback)(int value, void *user_data);
void for_each(const int *arr, size_t len, ForEachCallback cb, void *user_data);

#endif
