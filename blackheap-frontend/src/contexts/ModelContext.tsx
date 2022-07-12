import {createContext, Dispatch, useState, SetStateAction} from "react";

type ModelContextType = {
  jsonStr: string,
  setJsonStr: Dispatch<SetStateAction<string>>,
};

const ModelContext = createContext<ModelContextType | null>(null);

// TODO any
export const ModelProvider = ({children}: any) => {
  const [jsonStr, setJsonStr] = useState("");
  return <ModelContext.Provider value={{
    jsonStr,
    setJsonStr,
  }}>
    {children}
  </ModelContext.Provider>;
}

export default ModelContext;
