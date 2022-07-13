import {useContext} from "react";
import Model from "../types/Model";
import ModelContext from "../contexts/ModelContext";
import {BenchmarkType} from "../types/Model";
import {benchmark_type_str, is_read_op_str} from "../utils/ModelUtils";

type PlotViewProps = {
  benchmark_type: BenchmarkType,
  is_read_op: boolean,
}

const PlotView = ({benchmark_type, is_read_op}: PlotViewProps) => {
  const model: Model = useContext(ModelContext)!.json;
  const ourModel = model.find(el => el.benchmark_type === benchmark_type && el.is_read_op === is_read_op);
  return (
    <div> lol

    </div>
  );
}

export default PlotView;
