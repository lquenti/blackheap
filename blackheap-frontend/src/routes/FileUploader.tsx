import {FiUpload} from "react-icons/fi";
const FileUploader = () => {
  return (
    <div className="hero min-h-screen bg-base-200">
      <div className="hero-content text-center">
        <div className="max-w-md">
          <h1 className="text-5xl font-bold">Welcome to Blackheap!</h1>
          <p className="py-6">
            Please upload your performance model:
          </p>
          <button className="btn btn-primary">
            <FiUpload className="mr-3" />
            Select
          </button>
        </div>
      </div>
    </div>
  )
}


export default FileUploader;
