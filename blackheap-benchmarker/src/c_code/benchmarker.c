#include "benchmarker.h"

struct tuple tuple_add(const struct tuple *a, const struct tuple *b) {
  struct tuple result = { a->a + b->a, a->b + b->b };
  return result;
}

void inline_tuple_add(struct tuple *my, const struct tuple *other) {
  my->a += other->a;
  my->b += other->b;
}


