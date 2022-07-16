import { useContext } from "react";
import ModelContext from "../contexts/ModelContext";
import Model from "../types/Model";

import { benchmark_type_str, is_read_op_str } from "../utils/ModelUtils";

const Sidebar = () => {
  // TODO NULL OPERATOR
  const model: Model = useContext(ModelContext)!.json;
  const headlines = model.map(
    (m) =>
      `${benchmark_type_str(m.benchmark_type)}: ${is_read_op_str(m.is_read_op)}`
  );
  return (
    <ul className="menu p-4 overflow-y-auto w-80">
      {headlines.map((h, i) => (
        <li key={i}>{h}</li>
      ))}
    </ul>
  );
};

export default Sidebar;
