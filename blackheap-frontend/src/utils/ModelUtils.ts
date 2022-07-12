import {Interval, Constant, Linear, ConstantLinear, BenchmarkType, Analysis} from '../types/Model';

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
const evaluate = (f: {ConstantLinear: ConstantLinear} | {Linear: Linear}, x: number): number => {
  if ("ConstantLinear" in f) return evaluate_constant_linear(f.ConstantLinear, x);
  if ("Linear" in f) return evaluate_linear(f.Linear, x);
  throw new Error(`${f} couldn't get parsed`);
}

// ---
const get_all_maxima = (a: Analysis): Array<[number, number]> => {console.log(a); const res = a.kdes.map(k => k.global_maximum); console.log(res); console.log("---"); return res};

const get_all_maxima_x = (a: Analysis): Array<number> => get_all_maxima(a).map(([x, _]) => x);
const get_all_maxima_y = (a: Analysis): Array<number> => get_all_maxima(a).map(([_, y]) => y);

export {benchmark_type_str, is_read_op_str, equation_str, evaluate, get_all_maxima_x, get_all_maxima_y};
