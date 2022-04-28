const print_headline = (benchmark_type, is_read_op, div_name) => {
  const op = ((is_read_op) ? "read" : "write");
  document.getElementById(div_name).appendChild(document.createTextNode(`${benchmark_type}: ${op}`))
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
  const [biggest_access_size] = xs.slice(-1);
  // TODO: Verify if it actually looks like that by linspacing it
  const line = {
    x: [1, biggest_access_size],
    y: [f(1), f(biggest_access_size)],
    mode: 'lines+markers',
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
  console.log(div_name);
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
  kdes.forEach(k => {
    const row = document.createElement('div');
    row.classList.add('row');
    parent_div.appendChild(row);
    create_kde(k, row);
  });
};

const create_kde = ({xs, ys, access_size, cluster}, row) => {
  // cluster: {x_min, x_max, y_max}
  row.appendChild(document.createTextNode("test"));
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
}
