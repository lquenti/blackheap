import { Interval, Linear, ConstantLinear, BenchmarkType, BenchmarkKde } from '../types/Model';

const benchmark_type_str = (b: BenchmarkType): string => {
  switch (b) {
    case BenchmarkType.RandomUncached:
      return "Random Uncached";
    case BenchmarkType.SameOffset:
      return "Same Offset";
  }
}

const is_read_op_str = (is_read: boolean): string => (is_read) ? "Read" : "Write";

const interval_equation = (xs: Interval): string => {
  const lower = xs.lower === null ? "(-\\inf" : `[${xs.lower}`;
  const upper = xs.upper === null ? "\\inf)" : `${xs.upper}]`;
  return `${lower}, ${upper}`
}
const linear_equation = (linear: Linear): string => `f(x) = ${linear.slope} x + ${linear.y_intercept}`;
const constant_linear_equation = ({ constant, linear }: ConstantLinear): string => {
  const constant_function = `${constant.const_value}`;
  const constant_interval = interval_equation(constant.valid_interval);

  const linear_function = `${linear.slope} x + ${linear.y_intercept}`;
  const linear_interval = interval_equation(linear.valid_interval);

  return `
  f(x) =
  \\begin{cases}
  ${constant_function} & x \\in ${constant_interval}\\\\
  ${linear_function} & x \\in ${linear_interval}
  \\end{cases}
  `
}
const equation_str = (f: { ConstantLinear: ConstantLinear } | { Linear: Linear }): string => {
  if ("ConstantLinear" in f) return constant_linear_equation(f.ConstantLinear);
  if ("Linear" in f) return linear_equation(f.Linear);
  throw new Error(`${f} couldn't get parsed`);
}

// Evaluation stuff
const in_interval = (xs: Interval, x: number): boolean => {
  if (xs.lower !== null && xs.lower > x) return false;
  if (xs.upper !== null && xs.upper < x) return false;
  return true;
}
const evaluate_constant_linear = (f: ConstantLinear, x: number): number => {
  if (in_interval(f.linear.valid_interval, x)) return evaluate_linear(f.linear, x);
  if (in_interval(f.constant.valid_interval, x)) return f.constant.const_value;
  throw new Error(`${x} was not in any interval defined by ${f}`);
};
const evaluate_linear = (f: Linear, x: number): number => f.slope * x + f.y_intercept;
const evaluate = (f: { ConstantLinear: ConstantLinear } | { Linear: Linear }, x: number): number => {
  if ("ConstantLinear" in f) return evaluate_constant_linear(f.ConstantLinear, x);
  if ("Linear" in f) return evaluate_linear(f.Linear, x);
  throw new Error(`${f} couldn't get parsed`);
}

const get_max_for_access_sizes = (kdes: Array<BenchmarkKde>): { xs: Array<number>, ys: Array<number> } => {
  let xs = [], ys = [];
  for (const k of kdes) {
    xs.push(k["access_size"]);
    ys.push(k["global_maximum"][0]);
  }
  return { xs, ys };
}

export { benchmark_type_str, is_read_op_str, equation_str, evaluate, get_max_for_access_sizes };
