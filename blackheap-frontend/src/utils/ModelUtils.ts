import {Interval, Constant, Linear, ConstantLinear, BenchmarkType} from '../types/Model';

const benchmark_type_str = (b: BenchmarkType): string => {
  switch (b) {
    case BenchmarkType.RandomUncached:
      return "Random Uncached";
    case BenchmarkType.SameOffset:
      return "Same Offset";
  }
}

const is_read_op_str = (is_read: boolean): string => (is_read) ? "read" : "write";

// TODO: LaTeX
const interval_equation = (xs: Interval): string => {
  const lower = xs.lower === null ? "(-inf" : `[${xs.lower}`;
  const upper = xs.upper === null ? "inf)" : `${xs.upper})`;
  return `${lower}, ${upper}`
}
const constant_equation = (f: Constant): string => `x |-> ${f.const_value}, ${interval_equation(f.valid_interval)}`
const linear_equation = (f: Linear): string => `x |-> ${f.slope}*x + ${f.y_intercept}, ${interval_equation(f.valid_interval)}`
const constant_linear_equation = (f: ConstantLinear): string => `${constant_equation(f.constant)}----${linear_equation(f.linear)}`
const equation_str = (f: {ConstantLinear: ConstantLinear} | {Linear: Linear}): string => {
  if ("ConstantLinear" in f) {
    return constant_linear_equation(f.ConstantLinear);
  }
  if ("Linear" in f) {
    return linear_equation(f.Linear);
  }
  throw new Error(`${f} couldn't get parsed`);
}

export {benchmark_type_str, is_read_op_str, equation_str}
