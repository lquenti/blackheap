import { useContext } from "react";
import Model from "../types/Model";
import ModelContext from "../contexts/ModelContext";
import { BenchmarkType } from "../types/Model";
import {
  benchmark_type_str,
  equation_str,
  is_read_op_str,
} from "../utils/ModelUtils";
import Formula from "../components/Formula";
import KdePlot from "../components/KdePlot";
import SingleFunctionPlot from "../components/SingleFunctionPlot";

type PlotViewProps = {
  benchmark_type: BenchmarkType;
  is_read_op: boolean;
};

const PlotView = ({ benchmark_type, is_read_op }: PlotViewProps) => {
  // TODO NULL OPERATOR
  const model: Model = useContext(ModelContext)!.json;
  const ourModel = model.find(
    (el) => el.benchmark_type === benchmark_type && el.is_read_op === is_read_op
  )!;

  return (
    <div className="mx-auto max-w-2xl">
      <h1 className="text-center text-4xl mt-3 mb-9">
        {benchmark_type_str(benchmark_type)}: {is_read_op_str(is_read_op)}{" "}
        Operations
      </h1>
      {/* Formula */}
      <div>
        <Formula tex={equation_str(ourModel.model)} />
      </div>
      {/* Function plot */}
      <div>
        <SingleFunctionPlot a={ourModel} />
      </div>
      {/* Raw data TODO MAKE OWN COMPONENT */}
      <div className="overflow-x-auto my-9">
        <table className="table table-compact table-zebra w-full">
          <thead>
            <tr>
              <th>Access Size</th>
              <th>Approximated access time</th>
            </tr>
          </thead>
          <tbody>
            {ourModel.kdes.map((k, i) => (
              <tr key={`kde-table-${i}`}>
                <th>{k.access_size}</th>
                <th>{k.global_maximum[0]}</th>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
      {/* Plotting of each KDE*/}
      {ourModel.kdes.map((k, i) => (
        <KdePlot key={`kde-${i}`} k={k} />
      ))}
    </div>
  );
};

export default PlotView;
