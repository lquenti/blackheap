import { createContext, Dispatch, useState, SetStateAction } from "react";
import Model from "../types/Model";

// TODO ANY
type ModelContextType = {
  json: Model;
  setJson: Dispatch<SetStateAction<any>>;
};

const ModelContext = createContext<ModelContextType | null>(null);

// TODO any
export const ModelProvider = ({ children }: any) => {
  const [json, setJson] = useState([]);
  return (
    <ModelContext.Provider
      value={{
        json,
        setJson,
      }}
    >
      {children}
    </ModelContext.Provider>
  );
};

export default ModelContext;
