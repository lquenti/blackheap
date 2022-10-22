

// TODO: CREATE ONE SINGLE COMPONENT OUT

import { useEffect, useState } from "react";
import { useFilePicker } from "use-file-picker";

// OF BOTH UPLOADERS
const Evaluation = () => {
  const [openFileSelector, { filesContent }] = useFilePicker({
    accept: ".csv",
    limitFilesConfig: {max: 1},
  });
  const [enableRedirect, setEnableRedirect] = useState(false);
  
  useEffect(() => {
    if (filesContent.length !== 0) {
      console.log(filesContent[0].content);
    }
  }, [filesContent]);

  return (
    <div className="hero min-h-screen bg-base-100">
    <div className="hero-content text-center text-base-content">
      <div className="max-w-md">
        <p className="py-6 text-3xl">Upload your Records <a href="https://lquenti.github.io/blackheap/book/SingleNode.html">(Documentation)</a></p>
        <button onClick={openFileSelector} className="btn btn-primary">
          Select
        </button>
      </div>
    </div>
  </div>
  );
};
export default Evaluation;
