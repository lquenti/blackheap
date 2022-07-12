enum BenchmarkType {
  RandomUncached = "RandomUncached",
  SameOffset = "SameOffset",
};

type Cluster = {
  xs: Array<number>,
  ys: Array<number>,
  maximum: [number, number],
}

type BenchmarkKde = {
  access_size: number,
  xs: Array<number>,
  ys: Array<number>,
  significant_cluster: Array<Cluster>,
  global_maximum: [number, number],
};

type Interval = {
  lower: number | null,
  upper: number | null,
}

type Constant = {
  valid_interval: Interval,
  const_value: number,
}

type Linear = {
  valid_interval: Interval,
  slope: number,
  y_intercept: number,
}

type ConstantLinear = {
  constant: Constant,
  linear: Linear,
}


type Analysis = {
  benchmark_type: BenchmarkType,
  is_read_op: boolean,
  kdes: Array<BenchmarkKde>,
  model: {model: ConstantLinear | Linear},
};

export type {BenchmarkType, Cluster, BenchmarkKde, Interval, Constant, Linear, ConstantLinear, Analysis};

type Model = Array<Analysis>;
export default Model;
