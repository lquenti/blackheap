import { FiLogOut } from "react-icons/fi";
import { GoThreeBars } from "react-icons/go";

import Blackheap from "../components/Blackheap";
import { Link, NavLink, Outlet } from "react-router-dom";

const Dashboard = () => {
  return (
    <div className="drawer drawer-mobile text-base-content">
      <input id="my-drawer-2" type="checkbox" className="drawer-toggle" />
      <div className="drawer-content bg-base-100">
        <label
          htmlFor="my-drawer-2"
          className="btn btn-primary drawer-button lg:hidden m-3 px-3"
        >
          <GoThreeBars className="text-xl" />
        </label>
        <Outlet />
      </div>
      <div className="drawer-side">
        <label htmlFor="my-drawer-2" className="drawer-overlay"></label>
        <ul className="menu py-4 overflow-y-auto w-80 bg-base-300 text-base-content">
          <h2 className="text-2xl text-center pb-5">
            <Blackheap />
          </h2>
          <div className="py-3">
            <h3 className="text-lg font-semibold ml-3">Data Views</h3>
            <hr className="my-2" />
          </div>
          <li>
            <NavLink to="overview" end>
              Overview
            </NavLink>
          </li>
          <li>
            <NavLink to="randomread">Random Uncached: Read</NavLink>
          </li>
          <li>
            <NavLink to="randomwrite">Random Uncached: Write</NavLink>
          </li>
          <li>
            <NavLink to="offsetread">Same Offset: Read</NavLink>
          </li>
          <li>
            <NavLink to="offsetwrite">Same Offset: Write</NavLink>
          </li>
          <div className="py-3">
            <h3 className="text-lg font-semibold ml-3">Evaluation</h3>
            <hr className="my-2" />
          </div>
          <li>
            <NavLink to="evaluation">New Measurements</NavLink>
          </li>
          <li style={{ marginTop: "auto" }}>
            <Link to="/">
              <FiLogOut /> Logout
            </Link>
          </li>
        </ul>
      </div>
    </div>
  );
};

export default Dashboard;
