#include "clib.h"

#include <math.h>
#include <stdlib.h>

double point_norm(const Point *p) {
    return sqrt(p->x * p->x + p->y * p->y);
}

double sum_array(const double *data, size_t len) {
    double total = 0.0;
    for (size_t i = 0; i < len; i++) {
        total += data[i];
    }
    return total;
}

struct Counter {
    int value;
};

Counter *counter_new(int start) {
    Counter *c = malloc(sizeof(Counter));
    c->value = start;
    return c;
}

void counter_inc(Counter *c) {
    c->value += 1;
}

int counter_get(const Counter *c) {
    return c->value;
}

void counter_free(Counter *c) {
    free(c);
}

void for_each(const int *arr, size_t len, ForEachCallback cb, void *user_data) {
    for (size_t i = 0; i < len; i++) {
        cb(arr[i], user_data);
    }
}
