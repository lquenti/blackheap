import {BenchmarkKde} from '../types/Model';
import Plot from "react-plotly.js";

type KdePlotProps = {
  k: BenchmarkKde,
}

// TODO: DONT HARDCODE COLOURS
const KdePlot = ({k}: KdePlotProps) => (
  <Plot
    data={[{
      x: k.xs,
      y: k.ys,
      type: 'scatter',
      marker: {color: '#f3cc30'},
    }]}
    layout={{
      autosize: true,
      title: `Access Size: ${k.access_size} bytes`,
      paper_bgcolor: '#2d1b69',
      //paper_bgcolor: '#f00',
      plot_bgcolor: '#251655',
      margin: {
        t: 30,
        l: 30,
        r: 30,
        b: 30
      },
      font: {
        color: "#f9f7fd"
      }
    }}
  />
)

export default KdePlot;
