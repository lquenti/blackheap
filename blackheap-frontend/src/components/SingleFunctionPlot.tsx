import _ from "lodash";
import Plot from "react-plotly.js";
import { Analysis } from "../types/Model";
import { evaluate, get_max_for_access_sizes } from "../utils/ModelUtils";

const SingleFunctionPlot = ({ a }: { a: Analysis }) => {
  const { xs, ys } = get_max_for_access_sizes(a.kdes);
  const scatter = {
    x: xs,
    y: ys,
    mode: 'markers',
    name: 'Maxima of all KDEs',
    marker: { color: '#E779C1' },
  };
  const smallest_access_size = xs[0];
  const [biggest_access_size] = xs.slice(-1);
  const numberOfSteps = 150;
  const stepsize = (biggest_access_size - smallest_access_size) / numberOfSteps;
  const lgs = _.range(smallest_access_size, biggest_access_size, stepsize);
  const line = {
    x: lgs,
    y: lgs.map(x => evaluate(a.model, x)),
    mode: 'lines',
    name: 'Linearly interpolated function',
    marker: { color: '#f3cc30' },
  };
  const data = [scatter, line];
  return (
    <Plot
      data={data}
      layout={{
        autosize: true,
        title: 'Model Overview',
        paper_bgcolor: '#2d1b69',
        plot_bgcolor: '#251655',
        margin: {
          t: 50,
          l: 80,
          r: 80,
          b: 50
        },
        font: {
          color: "#f9f7fd"
        },
        xaxis: {
          type: 'log' as const,
          autorange: true,
          title: {
            text: "Access Size in Bytes" as const,
          },
          tickformat: 'f' as const,
        },
        yaxis: {
          type: 'log' as const,
          autorange: true,
          title: {
            text: "Expected Speed in Seconds" as const,
          },
          tickformat: 'f' as const,
        }
      }}
    />
  );
}

export default SingleFunctionPlot;
