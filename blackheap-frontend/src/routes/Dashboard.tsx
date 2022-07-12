import {useContext} from 'react';
import Plot from 'react-plotly.js';

import Blackheap from '../components/Blackheap';
import ModelContext from '../contexts/ModelContext';

const Dashboard = () => {
  // TODO NULL OPERATOR
  const {jsonStr} = useContext(ModelContext)!;
  return (
    <div className="drawer drawer-mobile text-base-content">
      <input id="my-drawer-2" type="checkbox" className="drawer-toggle" />
      <div className="drawer-content flex flex-col items-center justify-center bg-base-100">
        {/* Page content here */}
        <div>
          {jsonStr}
        </div>
        <div>
          <Plot
            data={[
              {
                x: [1, 2, 3],
                y: [2, 6, 3],
                type: 'scatter',
                mode: 'lines+markers',
                marker: {color: 'red'},
              },
              {type: 'bar', x: [1, 2, 3], y: [2, 5, 3]},
            ]}
            layout={{title: 'A Fancy Plot'}}
          />
        </div>

        <label htmlFor="my-drawer-2" className="btn btn-primary drawer-button lg:hidden">Open drawer</label>

      </div>
      <div className="drawer-side">
        <label htmlFor="my-drawer-2" className="drawer-overlay"></label>
        <ul className="menu p-4 overflow-y-auto w-80 bg-base-300 text-base-content">
          <h1 className="text-2xl text-center"><Blackheap /></h1>
          {/* Sidebar content here */}
          <li><a>Sidebar Item 1</a></li>
          <li><a>Sidebar Item 2</a></li>
        </ul>

      </div>
    </div>
  );
};

export default Dashboard;
