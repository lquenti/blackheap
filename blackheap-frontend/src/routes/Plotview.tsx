import {BenchmarkType} from "../types/Model";
import {benchmark_type_str, is_read_op_str} from "../utils/ModelUtils";

type PlotViewProps = {
  benchmark_type: BenchmarkType,
  is_read_op: boolean,
}

const PlotView = ({benchmark_type, is_read_op}: PlotViewProps) => {
  return (
    <div>
      Plotview: {benchmark_type_str(benchmark_type)}: {is_read_op_str(is_read_op)}
    </div>
  );
}

export default PlotView;
