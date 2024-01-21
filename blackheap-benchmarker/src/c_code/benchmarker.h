#ifndef BLACKHEAP_BENCHMARKER_BENCHMARER_H
#define BLACKHEAP_BENCHMARKER_BENCHMARER_H

struct tuple {
  int a;
  int b;
};

struct tuple tuple_add(const struct tuple *a, const struct tuple *b);
void inline_tuple_add(struct tuple *my, const struct tuple *other);

#endif
