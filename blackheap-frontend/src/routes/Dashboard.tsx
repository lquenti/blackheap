import {useContext} from 'react';
import {FiLogOut} from 'react-icons/fi';

import Model from '../types/Model';
import Blackheap from '../components/Blackheap';
import ModelContext from '../contexts/ModelContext';

const Dashboard = () => {
  // TODO NULL OPERATOR
  const model: Model = useContext(ModelContext)!.json;
  return (
    <div className="drawer drawer-mobile text-base-content">
      <input id="my-drawer-2" type="checkbox" className="drawer-toggle" />
      <div className="drawer-content flexitems-center justify-center bg-base-100">
        {/* Page content here */}
        <label htmlFor="my-drawer-2" className="btn btn-primary drawer-button lg:hidden">Open drawer</label>

      </div>
      <div className="drawer-side">
        <label htmlFor="my-drawer-2" className="drawer-overlay"></label>
        <ul className="menu py-4 overflow-y-auto w-80 bg-base-300 text-base-content">
          <h2 className="text-2xl text-center pb-5"><Blackheap /></h2>
          <div className="py-3">
            <h3 className="text-lg font-semibold ml-3">Data Views</h3>
            <hr className="my-2" />
          </div>
          <li><a className="active">Overview</a></li>
          <li><a>Random Uncached: Read</a></li>
          <li><a>Random Uncached: Write</a></li>
          <li><a>Same Offset: Read</a></li>
          <li><a>Same Offset: Write</a></li>
          <div className="py-3">
            <h3 className="text-lg font-semibold ml-3">Evaluation</h3>
            <hr className="my-2" />
          </div>
          <li><a>New Measurements</a></li>
          <li style={{marginTop: "auto"}}><a><FiLogOut /> Logout</a></li> {/* TODO: Only refresh to root */}
        </ul>
      </div>
    </div>
  );
};

export default Dashboard;
