import { useEffect, useContext, useState } from "react";
import { FiUpload } from "react-icons/fi";
import { Navigate } from "react-router-dom";

import Model from "../types/Model";
import Blackheap from "../components/Blackheap";
import { useFilePicker } from "use-file-picker";
import ModelContext from "../contexts/ModelContext";

const FileUploader = () => {
  const [openFileSelector, { filesContent }] = useFilePicker({
    accept: ".json",
    limitFilesConfig: { max: 1 },
  });
  const [enableRedirect, setEnableRedirect] = useState(false);

  // TODO non null operator
  const { setJson } = useContext(ModelContext)!;

  useEffect(() => {
    if (filesContent.length !== 0) {
      const parsed: Model = JSON.parse(filesContent[0].content);
      setJson(parsed);
      setEnableRedirect(true);
    }
  }, [filesContent, setJson]);

  return (
    <div className="hero min-h-screen bg-base-100">
      <div className="hero-content text-center text-base-content">
        <div className="max-w-md">
          <h1 className="text-5xl font-bold">
            Welcome to <Blackheap />
          </h1>
          <p className="py-6">Please upload your performance model (Model.json):</p>
          <button onClick={openFileSelector} className="btn btn-primary">
            <FiUpload className="mr-3" />
            Select
          </button>
        </div>
      </div>
      {enableRedirect && <Navigate to="/dashboard/overview" />}
    </div>
  );
};

export default FileUploader;
