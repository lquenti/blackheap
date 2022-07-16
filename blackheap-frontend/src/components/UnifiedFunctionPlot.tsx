import _ from "lodash";
import { useContext } from "react";
import Plot from "react-plotly.js";
import ModelContext from "../contexts/ModelContext";
import Model from "../types/Model";
import {
  benchmark_type_str,
  evaluate,
  is_read_op_str,
} from "../utils/ModelUtils";

// TODO Reduce redundancy with other Plots
// TODO create base plot
const UnifiedFunctionPlot = () => {
  const model: Model = useContext(ModelContext)!.json;
  let data = [];
  for (const m of model) {
    // range
    const smallest_access_size = m.kdes[0].access_size;
    const biggest_access_size = m.kdes.slice(-1)[0].access_size;
    const numberOfSteps = 150;
    const stepsize =
      (biggest_access_size - smallest_access_size) / numberOfSteps;
    const lgs = _.range(smallest_access_size, biggest_access_size, stepsize);

    // Generate line
    const line = {
      x: lgs,
      y: lgs.map((x) => evaluate(m.model, x)),
      mode: "lines",
      // TODO outsource this string gen
      name: `${benchmark_type_str(m.benchmark_type)}: ${is_read_op_str(
        m.is_read_op
      )}`,
      marker: {
        color: `#${[1, 2, 3, 4, 5, 6]
          .map((_) => "0123456789ABCDEF"[Math.floor(Math.random() * 16)])
          .join("")}`,
      }, // TODO COLOUR
    };
    data.push(line);
  }
  return (
    <Plot
      data={data}
      layout={{
        autosize: true,
        title: "Model Overview",
        paper_bgcolor: "#2d1b69",
        plot_bgcolor: "#251655",
        margin: {
          t: 50,
          l: 80,
          r: 80,
          b: 50,
        },
        font: {
          color: "#f9f7fd",
        },
        xaxis: {
          type: "log" as const,
          autorange: true,
          title: {
            text: "Access Size in Bytes" as const,
          },
          tickformat: "f" as const,
        },
        yaxis: {
          type: "log" as const,
          autorange: true,
          title: {
            text: "Expected Speed in Seconds" as const,
          },
          tickformat: "f" as const,
        },
      }}
    />
  );
};
export default UnifiedFunctionPlot;
