import {BenchmarkKde} from '../types/Model';
import Plot from "react-plotly.js";

type KdePlotProps = {
  k: BenchmarkKde,
}

// TODO: DONT HARDCODE COLOURS
const KdePlot = ({k}: KdePlotProps) => {
  console.log(k);
  const colours = ['#E779C1', '#fff'];
  const graph = {
    x: k.xs,
    y: k.ys,
    mode: 'lines',
    name: 'KDE',
    marker: {color: '#f3cc30'},
  }

  // Maxima
  let maxima_x = [], maxima_y = [];
  for (const sc of k.significant_clusters) {
    maxima_x.push(sc["maximum"][0]);
    maxima_y.push(sc["maximum"][1]);
  }
  const maxima = {
    x: maxima_x,
    y: maxima_y,
    mode: 'markers',
    name: 'Maxima of each cluster'
  };
  // the clusters themselves
  // https://plotly.com/javascript/shapes/
  const clusters = k.significant_clusters.map((sc, i) => {
    return {
      type: "rect" as const,
      xref: 'x' as const,
      yref: 'paper' as const,
      x0: sc["xs"][0],
      y0: 0,
      x1: sc["xs"].slice(-1)[0],
      y1: 1,
      fillcolor: colours[i % colours.length],
      opacity: 0.4,
      line: {
        width: 1,
      },
    };
  })
  const data = [graph, maxima];
  return (
    <Plot
      data={data}
      layout={{
        autosize: true,
        title: `Access Size: ${k.access_size} bytes`,
        paper_bgcolor: '#2d1b69',
        plot_bgcolor: '#251655',
        margin: {
          t: 30,
          l: 30,
          r: 30,
          b: 30
        },
        font: {
          color: "#f9f7fd"
        },
        shapes: clusters
      }}
    />
  );
}

export default KdePlot;
