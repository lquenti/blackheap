import {useEffect} from 'react';
import {FiUpload} from "react-icons/fi";

import {useFilePicker} from "use-file-picker";

const FileUploader = () => {
  const [openFileSelector, {filesContent}] = useFilePicker({
    accept: ".json",
    limitFilesConfig: {max: 1},
  });

  useEffect(() => {
    if (filesContent.length !== 0) {
      console.log(filesContent[0].content);
    }
  }, [filesContent]);



  return (
    <div className="hero min-h-screen bg-base-200">
      <div className="hero-content text-center">
        <div className="max-w-md">
          <h1 className="text-5xl font-bold">Welcome to Blackheap!</h1>
          <p className="py-6">
            Please upload your performance model:
          </p>
          <button onClick={openFileSelector} className="btn btn-primary">
            <FiUpload className="mr-3" />
            Select
          </button>
        </div>
      </div>
    </div>
  )
}


export default FileUploader;
