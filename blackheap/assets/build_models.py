import csv
import os
import matplotlib.pyplot as plt
import numpy as np
import scipy.stats as st
import sklearn.cluster as sklc

from dataclasses import dataclass, field
from typing import Dict, List, Tuple, Union

access_size = int


@dataclass
class Benchmark:
    raw_data: List[float]
    min_val: int
    max_val: int
    kde: st._kde.gaussian_kde

    @classmethod
    def from_values(cls, values, *args, **kwargs):
        kde = st.gaussian_kde(np.array(values), *args, **kwargs)
        min_val = min(values)
        max_val = max(values)
        return cls(values, min_val, max_val, kde)


@dataclass
class Measurements:
    name: str
    is_read: bool
    data: Dict[access_size, Benchmark] = field(default_factory=dict)

    def __str__(self):
        operation_type = "Read" if self.is_read else "Write"
        return f"Measurements(Name: {self.name}, Operation: {operation_type})"

    def __repr__(self):
        operation_type = "Read" if self.is_read else "Write"
        return f"Measurements(name='{self.name}', is_read={self.is_read}, data=<{len(self.data)} items>)"


@dataclass
class Cluster:
    left_boundary: float
    right_boundary: float


@dataclass
class Model:
    benchmark_type: str
    is_read_op: bool
    slope: float
    y_intercept: float
    left_bound: int
    right_bound: int

    @classmethod
    def new_linear(cls, benchmark_type, is_read_op, xs, ys):
        slope, intercept, _, _, _ = st.linregress(xs, ys)
        return cls(benchmark_type, is_read_op, slope, intercept, 0, 0)

    @classmethod
    def new_constlinear(
        cls, benchmark_type, is_read_op, xs, ys, cutoff=4096
    ):
        # Calculate the constant part
        const_ys = [y for x, y in zip(xs, ys) if x <= cutoff]
        constant = max(const_ys) if const_ys else 0

        # Calculate the linear part for x > cutoff
        linear_xs = [x for x in xs if x > cutoff]
        linear_ys = [y for x, y in zip(xs, ys) if x > cutoff]
        slope, intercept, _, _, _ = (
            st.linregress(linear_xs, linear_ys)
            if linear_xs and linear_ys
            else (0, constant, 0, 0, 0)
        )

        # First part is constant
        model_const = cls(benchmark_type, is_read_op, 0, constant, 0, cutoff)
        # Second part is linear
        model_linear = cls(benchmark_type, is_read_op, slope, intercept, cutoff, 0)
        return model_const, model_linear

    def to_iofs_csv_str(self):
        is_read_op_int = 1 if self.is_read_op else 0
        return f"{self.benchmark_type},{is_read_op_int},{self.slope},{self.y_intercept},{self.left_bound},{self.right_bound}"

    @classmethod
    def all_to_csv(cls, models):
        header = "benchmark_type,is_read_op,slope,y_intercept,left_bound,right_bound"
        csv_lines = [header]
        csv_lines += [model.to_iofs_csv_str() for model in models]
        return "\n".join(csv_lines)

    @classmethod
    def all_from_csv(cls, file_path):
        models = []
        with open(file_path, mode='r', encoding='utf-8') as csvfile:
            csv_reader = csv.DictReader(csvfile)
            for row in csv_reader:
                benchmark_type = row['benchmark_type']
                is_read_op = bool(int(row['is_read_op']))
                slope = float(row['slope'])
                y_intercept = float(row['y_intercept'])
                left_bound = int(row['left_bound'])
                right_bound = int(row['right_bound'])
                
                model = cls(benchmark_type, is_read_op, slope, y_intercept, left_bound, right_bound)
                models.append(model)
        return models


def get_benchmark_dirs(root):
    # to exclude stuff like venvs
    is_benchmark_dir = lambda d: os.path.isdir(
        os.path.join(d, "read")
    ) and os.path.isdir(os.path.join(d, "write"))
    ret = []
    for name in os.listdir(root):
        full_path = os.path.join(root, name)
        if not os.path.isdir(full_path):
            continue
        if not is_benchmark_dir(full_path):
            continue
        ret.append(full_path)
    return ret


def load_benchmark_folder(dir_path: str) -> Tuple[Measurements, Measurements]:
    read_measurements = Measurements(name=os.path.basename(dir_path), is_read=True)
    write_measurements = Measurements(name=os.path.basename(dir_path), is_read=False)

    for operation in ["read", "write"]:
        operation_path = os.path.join(dir_path, operation)
        for file_name in os.listdir(operation_path):
            file_path = os.path.join(operation_path, file_name)
            file_size = int(file_name.replace(".txt", ""))
            with open(file_path, "r") as file:
                values = [float(line.strip()) for line in file.readlines()]

            if operation == "read":
                read_measurements.data[file_size] = Benchmark.from_values(values)
            else:
                write_measurements.data[file_size] = Benchmark.from_values(values)

    return read_measurements, write_measurements


def find_all_clusters_derivative(xs: List[float], ys: List[float]) -> List[Cluster]:
    minima = []

    assert len(xs) == len(ys)

    # the first point is definitely a minimum
    minima.append(xs[0])

    for i in range(1, len(xs) - 1):
        if (ys[i - 1] > ys[i]) and (ys[i] < ys[i + 1]):
            minima.append(xs[i])

    # The last point is definitely a minimum
    minima.append(xs[-1])

    clusters = []
    for i in range(len(minima) - 1):
        cluster = Cluster(left_boundary=minima[i], right_boundary=minima[i + 1])
        clusters.append(cluster)

    return clusters


def apply_cutoff_to_last_cluster(xs, ys, last_cluster, cutoff_threshold_ratio=0.05):
    # find max
    cluster_indices = np.where(
        (xs >= last_cluster.left_boundary) & (xs <= last_cluster.right_boundary)
    )[0]
    last_max_index = cluster_indices[0]
    last_max_y = ys[last_max_index]
    for index in cluster_indices:
        if ys[index] > last_max_y:
            last_max_y = ys[index]
            last_max_index = index

    cutoff_threshold = cutoff_threshold_ratio * last_max_y

    # Walk downhill from the last maximum until the density falls below the cutoff threshold
    for i in range(last_max_index, len(xs)):
        if ys[i] < cutoff_threshold:
            return xs[i]  # Return new right boundary when below threshold

    # If the threshold is never met, return the original right boundary
    return last_cluster.right_boundary


def find_significant_clusters_derivative(
    b: Benchmark,
    num_points=1000,
    significant_percentage=0.1,
    cutoff_threshold_ratio=0.05,
):
    xs = np.linspace(b.min_val, b.max_val, num_points)
    ys = b.kde(xs)
    all_clusters = find_all_clusters_derivative(xs, ys)
    # plot_kde_with_clusters(b.kde, b.min_val, b.max_val, all_clusters, num_points)

    # The rule is as follows:
    # A cluster is significant iff
    # (maximum - minimum) >= significant_percentage * global_maximum
    #
    # We merge the clusters until that is true.

    global_max = max(ys)
    significant_clusters = []

    for cluster in all_clusters:
        cluster_max = max(
            ys[(xs >= cluster.left_boundary) & (xs <= cluster.right_boundary)]
        )
        left_min = min(ys[xs == cluster.left_boundary])
        right_min = min(ys[xs == cluster.right_boundary])

        # Check if the cluster is significant
        if (
            cluster_max - min(left_min, right_min)
        ) >= significant_percentage * global_max:
            significant_clusters.append(cluster)
        else:
            # If not significant, merge with the next cluster if possible
            if significant_clusters:
                significant_clusters[-1].right_boundary = cluster.right_boundary
            else:
                # It is the first one
                significant_clusters.append(cluster)

    if not significant_clusters:
        return []

    # Apply cutoff to the last cluster if there are any significant clusters
    last_cluster = significant_clusters[-1]
    last_cluster.right_boundary = apply_cutoff_to_last_cluster(
        xs, ys, last_cluster, cutoff_threshold_ratio
    )

    # Find the biggest cluster
    biggest_cluster = None
    biggest_cluster_val = -1
    for cluster in significant_clusters:
        cluster_max = max(
            ys[(xs >= cluster.left_boundary) & (xs <= cluster.right_boundary)]
        )
        if cluster_max > biggest_cluster_val:
            biggest_cluster = cluster
            biggest_cluster_val = cluster_max

    return significant_clusters, biggest_cluster


def find_clusters_meanshift(b: Benchmark, num_points=1000, cutoff_threshold_ratio=0.05):
    xs = np.linspace(b.min_val, b.max_val, num_points)
    ys = b.kde(xs)
    xsys = [list(pair) for pair in zip(xs, ys)]
    clustering = sklc.MeanShift().fit(np.array(xsys))
    labels = clustering.labels_

    clusters = []
    for label in np.unique(labels):
        cluster_points = xs[labels == label]

        left_boundary = np.min(cluster_points)
        right_boundary = np.max(cluster_points)

        clusters.append(Cluster(left_boundary, right_boundary))

    # Ensure clusters are sorted by their left boundary to find the rightmost cluster
    clusters.sort(key=lambda cluster: cluster.left_boundary)

    if not clusters:
        return [], None

    # Apply cutoff to the last (rightmost) cluster if there are any clusters
    last_cluster = clusters[-1]
    new_right_boundary = apply_cutoff_to_last_cluster(
        xs, ys, last_cluster, cutoff_threshold_ratio
    )
    last_cluster.right_boundary = new_right_boundary

    # Find the biggest cluster
    biggest_cluster = None
    biggest_cluster_val = -1
    for cluster in clusters:
        cluster_max = max(
            ys[(xs >= cluster.left_boundary) & (xs <= cluster.right_boundary)]
        )
        if cluster_max > biggest_cluster_val:
            biggest_cluster = cluster
            biggest_cluster_val = cluster_max


    return clusters, biggest_cluster


def measurements_to_model(
    measurements: Measurements,
    use_derivative=True,
    use_linear=True,
    use_biggest=False,
    **kwargs,
):
    cluster_f = (
        find_significant_clusters_derivative
        if use_derivative
        else find_clusters_meanshift
    )
    model_f = Model.new_linear if use_linear else Model.new_constlinear

    xs = []
    ys = []
    for access_size, benchmark in measurements.data.items():
        significant_clusters, biggest_cluster = cluster_f(benchmark, **kwargs)
        if use_biggest and biggest_cluster is not None:
            # (second condition only to make linter happy, we know that we get at least one cluster :D)
            y = biggest_cluster.right_boundary
        else:
            y = significant_clusters[-1].right_boundary

        xs.append(access_size)
        ys.append(y)

    return model_f(
        benchmark_type=measurements.name,
        is_read_op=1 if measurements.is_read else 0,
        xs=xs,
        ys=ys,
    )


def measurements_to_linear_model_derivative(
    measurements: Measurements,
    num_points=1000,
    significant_percentage=0.1,
    cutoff_threshold_ratio=0.05,
    use_biggest=False,
):
    return measurements_to_model(
        measurements,
        use_derivative=True,
        use_linear=True,
        num_points=num_points,
        significant_percentage=significant_percentage,
        cutoff_threshold_ratio=cutoff_threshold_ratio,
        use_biggest=use_biggest,
    )


def measurements_to_constlinear_model_derivative(
    measurements: Measurements,
    num_points=1000,
    significant_percentage=0.1,
    cutoff_threshold_ratio=0.05,
    use_biggest=False,
):
    return measurements_to_model(
        measurements,
        use_derivative=True,
        use_linear=False,
        num_points=num_points,
        significant_percentage=significant_percentage,
        cutoff_threshold_ratio=cutoff_threshold_ratio,
        use_biggest=use_biggest,
    )


def measurements_to_linear_model_meanshift(
    measurements: Measurements,
    num_points=1000,
    cutoff_threshold_ratio=0.05,
    use_biggest=False,
):
    return measurements_to_model(
        measurements,
        use_derivative=False,
        use_linear=True,
        num_points=num_points,
        cutoff_threshold_ratio=cutoff_threshold_ratio,
        use_biggest=use_biggest,
    )


def measurements_to_constlinear_model_meanshift(
    measurements: Measurements,
    num_points=1000,
    cutoff_threshold_ratio=0.05,
    use_biggest=False,
):
    return measurements_to_model(
        measurements,
        use_derivative=False,
        use_linear=False,
        num_points=num_points,
        cutoff_threshold_ratio=cutoff_threshold_ratio,
        use_biggest=use_biggest,
    )


if __name__ == "__main__":
    print("Parsing arguments...")
    import argparse

    parser = argparse.ArgumentParser(
        description="Creates a model based on the measurements done by blackheap"
    )
    parser.add_argument(
        "--constlinear",
        action="store_true",
        help="If set, it will generate a constlinear model instead of a linear one",
    )
    parser.add_argument(
        "--meanshift",
        action="store_true",
        help="If set, it will use the Meanshift based clustering algorithm instead of the derivative based one. See the jupyter notebook for more explaination.",
    )
    parser.add_argument(
        "--usebiggest",
        action="store_true",
        help="If set, it will use the right bound of the biggest significant cluster instead of the last one",
    )
    parser.add_argument(
        "--cluster-significance",
        type=float,
        default=0.1,
        help="See the jupyter notebook for explaination",
    )
    parser.add_argument(
        "--last-cluster-threshold",
        type=float,
        default=0.05,
        help="See the jupyter notebook for explaination",
    )
    args = parser.parse_args()

    def pick_cluster_function_based_on_cli(args):
        if args.constlinear and args.meanshift:
            return lambda m: measurements_to_constlinear_model_meanshift(
                m,
                cutoff_threshold_ratio=args.last_cluster_threshold,
                use_biggest=args.usebiggest,
            )
        if args.constlinear and not args.meanshift:
            return lambda m: measurements_to_constlinear_model_derivative(
                m,
                significant_percentage=args.cluster_significance,
                cutoff_threshold_ratio=args.last_cluster_threshold,
                use_biggest=args.usebiggest,
            )
        if not args.constlinear and args.meanshift:
            return lambda m: measurements_to_linear_model_meanshift(
                m,
                cutoff_threshold_ratio=args.last_cluster_threshold,
                use_biggest=args.usebiggest,
            )
        if not args.constlinear and not args.meanshift:
            return lambda m: measurements_to_linear_model_derivative(
                m,
                significant_percentage=args.cluster_significance,
                cutoff_threshold_ratio=args.last_cluster_threshold,
                use_biggest=args.usebiggest,
            )
        assert False

    measurements_to_model_f = pick_cluster_function_based_on_cli(args)

    print("Loading all measurements in")
    script_dir = os.path.dirname(os.path.realpath(__file__))
    all_measurements = [
        load_benchmark_folder(x) for x in get_benchmark_dirs(script_dir)
    ]

    all_models = []
    for read, write in all_measurements:
        print(f"Processing: {read.name}")
        all_models.append(measurements_to_model_f(read))
        all_models.append(measurements_to_model_f(write))

    # If we had constlinear, it is a list of tuples, so we have to flatten it down
    all_models_flattened = []
    for item in all_models:
        if isinstance(item, tuple):
            all_models_flattened.extend(item)
        else:
            all_models_flattened.append(item)

    with open(f"model.csv", "w") as fp:
        fp.write(Model.all_to_csv(all_models_flattened))
    print(f"Model successfully saved as model.csv")
