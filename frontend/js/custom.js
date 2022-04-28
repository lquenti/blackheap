/* Begin helpers */
const get_random_colour = () => {
  const space = "0123456789ABCDEF";
  return `#${Array(6).fill().map(_ => space[Math.floor(Math.random() * 16)]).join('')}`
}
const convert_is_read_op = (is_read_op) => (is_read_op) ? "read" : "write";

// https://stackoverflow.com/a/40475362/9958281
const linspace = (startValue, stopValue, cardinality) => {
  let arr = [];
  const step = (stopValue - startValue) / (cardinality - 1);
  console.log(startValue, stopValue, cardinality);
  for (var i = 0; i < cardinality; i++) {
    arr.push(startValue + (step * i));
  }
  return arr;
}

const logspace = (startValue, stopValue, cardiality) => {
  return linspace(Math.log10(startValue), Math.log10(stopValue), cardiality).map(x => Math.pow(10, x));
}

/* End helpers */

const print_headline = (benchmark_type, is_read_op, div_name) => {
  document.getElementById(div_name).appendChild(document.createTextNode(`${benchmark_type}: ${convert_is_read_op(is_read_op)}`))
};

const print_model_function = (slope, intercept, div_name) => {
  document.getElementById(div_name).appendChild(document.createTextNode(`Linear Model: ${slope}*x+${intercept}`))
};

const plot_overview = (xs, ys, slope, intercept, div_name) => {
  const f = x => slope * x + intercept;
  const scatter = {
    x: xs,
    y: ys,
    mode: 'markers',
    name: 'Maxima of all KDEs',
  };
  const smallest_access_size = xs[0];
  const [biggest_access_size] = xs.slice(-1);
  const lgs = logspace(smallest_access_size, biggest_access_size, 150);

  const line = {
    x: lgs,
    y: lgs.map(f),
    mode: 'lines',
    name: 'Linearly interpolated function'
  };
  const data = [scatter, line];
  const layout = {
    xaxis: {
      text: 'Access Size in Bytes',
      type: 'log',
      autorange: 'true',
      rangemode: 'tozero',
      tickformat: 'f',
    },
    yaxis: {
      text: 'Expected Speed in sec',
      type: 'log',
      autorange: 'true',
      tickformat: 'e',
    },
    title: 'Model Overview',
  };
  Plotly.newPlot(div_name, data, layout);
};

const create_table = (xs, ys, div_name) => {
  const parent_div = document.getElementById(div_name);
  const table = document.createElement('table');
  const thead = document.createElement('thead');

  // create header
  const head_xs = document.createElement('th');
  head_xs.appendChild(document.createTextNode('Access size in Bytes'));
  const head_ys = document.createElement('th');
  head_ys.appendChild(document.createTextNode('Time in Seconds'));
  thead.appendChild(head_xs);
  thead.appendChild(head_ys);
  table.appendChild(thead);

  // create body
  for (let row = 0; row < xs.length; row++) {
    const tr = table.insertRow();
    const access_size = tr.insertCell();
    const time = tr.insertCell();
    access_size.appendChild(document.createTextNode(xs[row]));
    time.appendChild(document.createTextNode(ys[row]));
  }
  parent_div.appendChild(table);
};

const create_kdes = (kdes, div_name) => {
  const parent_div = document.getElementById(div_name);
  const headline = document.createElement('h4');
  headline.appendChild(document.createTextNode('KDEs'))
  parent_div.appendChild(headline);

  for (const kde of kdes) {
    const row = document.createElement('div');
    row.classList.add('row');
    parent_div.appendChild(row);
    create_kde(kde, row);
  }
};

const create_kde = ({access_size, significant_clusters, xs, ys}, row) => {
  // create headline
  const headline = document.createElement("h4");
  headline.appendChild(document.createTextNode(`Access Size: ${access_size}`));

  // create another div for putting the graph in
  const plot_div = document.createElement('div');

  // make row unique to pass to plotly
  const row_name = `kde-${access_size}`;
  plot_div.id = row_name;

  // append everything
  row.appendChild(headline);
  row.appendChild(plot_div);

  // plot...
  // the kde itself
  const graph = {
    x: xs,
    y: ys,
    mode: 'lines',
    name: 'KDE',
  };

  // the maxima
  let maxima_x = [], maxima_y = [];
  for (const sc of significant_clusters) {
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
  const clusters = significant_clusters.map(sc => {
    return {
      type: 'rect',
      xref: 'x',
      yref: 'paper',
      x0: sc["xs"][0],
      y0: 0,
      x1: sc["xs"].slice(-1)[0],
      y1: 1,
      fillcolor: get_random_colour(),
      opacity: 0.4,
      line: {
        width: 1,
      },
    };
  })

  const data = [graph, maxima];
  const layout = {
    xaxis: {
      text: 'time in seconds',
    },
    yaxis: {
      text: 'Estimated Probability',
    },
    shapes: clusters,
  };
  Plotly.newPlot(row_name, data, layout);
};

const get_model_from_json = (j, benchmark_type, is_read_op) => {
  for (const m of j) {
    if (m["benchmark_type"] === benchmark_type && m["is_read_op"] == is_read_op) {
      return m;
    }
  }
  console.error("Benchmark not found...");
}

const get_max_for_access_sizes = kdes => {
  let xs = [], ys = [];
  for (const k of kdes) {
    xs.push(k["access_size"]);
    ys.push(k["global_maximum"][0]);
  }
  return {xs, ys};
}

const single_model_main = (j, wanted_benchmark_type, wanted_is_read_op) => {
  const {benchmark_type, is_read_op, kdes, linear_model} = get_model_from_json(j, wanted_benchmark_type, wanted_is_read_op);
  // TODO sort access sizes
  const {xs, ys} = get_max_for_access_sizes(kdes);

  print_headline(benchmark_type, is_read_op, 'headline');
  print_model_function(linear_model.a, linear_model.b, 'model');
  plot_overview(xs, ys, linear_model.a, linear_model.b, 'overview');
  create_table(xs, ys, 'raw-data');
  create_kdes(kdes, 'kdes');
  document.title = `${benchmark_type}: ${convert_is_read_op(is_read_op)} - Report`
};

//////////////////////////////////////////////////////////////////////////////////////////////////

const print_all_functions = (j, div_name) => {
  const parent_div = document.getElementById(div_name);
  for (const [index, model] of j.entries()) {
    const row = document.createElement('div');
    row.classList.add('row');
    const row_name = `model-${index}`;
    row.id = row_name;

    const slope = model["linear_model"]["a"];
    const intercept = model["linear_model"]["b"];

    const headline = document.createElement('h4');
    headline.appendChild(document.createTextNode(`${model["benchmark_type"]}: ${convert_is_read_op(model["is_read_op"])}`));

    const p = document.createElement('p');
    const f = `${slope}*x + ${intercept}`;

    p.appendChild(document.createTextNode(f));
    row.appendChild(headline);
    row.appendChild(p);
    parent_div.appendChild(row);
  }
}

const plot_joined_functions = (j, div_name) => {
  let lines = [];
  for (const model of j) {
    const smallest_access_size = model["kdes"][0].access_size;
    console.log(smallest_access_size);
    const biggest_access_size = model["kdes"].slice(-1)[0].access_size;
    const slope = model["linear_model"]["a"];
    const intercept = model["linear_model"]["b"];
    const model_name = `${model["benchmark_type"]}: ${convert_is_read_op(model["is_read_op"])}`;
    const random_colour = get_random_colour();
    const f = x => slope * x + intercept;
    const xs = logspace(smallest_access_size, biggest_access_size, 150);
    console.log(xs);
    const line = {
      x: xs,
      y: xs.map(f),
      mode: 'lines',
      name: model_name,
      marker: {
        color: random_colour,
        line: {
          color: get_random_colour,
        }
      }
    };
    lines.push(line);
  }
  const layout = {
    xaxis: {
      text: 'Access Size in Bytes',
      type: 'log',
      autorange: 'true',
      rangemode: 'tozero',
      tickformat: 'f'
    },
    yaxis: {
      text: 'Expected Speed in sec',
      type: 'log',
      autorange: 'true',
      tickformat: 'e',
    },
    title: 'Models Overview'
  };
  Plotly.newPlot(div_name, lines, layout);
}

const model_summary_main = j => {
  print_all_functions(j, 'all-models');
  plot_joined_functions(j, 'model-plot');

  document.title = "Summary of all Models";
};

/////////////////////////////////////////////



const use_model_main = j => {
  const {number_of_classified, number_of_unclassified, read_bytes_sec, write_bytes_sec} = j;
  document.title = "Report of recorded data";
}
