import { Dispatch, useEffect } from "react";
import { useFilePicker } from "use-file-picker";
import PreloadeeRecords from "../../types/PreloadeeRecords";
import { parsePreloadeeData } from "../../utils/PreloadeeRecordsUtils";

const EvaluationUploader = ({setPreloadeeRecords}: any) => {
    const [openFileSelector, { filesContent }] = useFilePicker({
        accept: ".csv",
        limitFilesConfig: {max: 1},
      });

      useEffect(() => {
        if (filesContent.length !== 0) {
          const unparsedCsv: string = filesContent[0].content;
          const parsed: PreloadeeRecords = parsePreloadeeData(unparsedCsv);
          setPreloadeeRecords(parsed);
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
}

export default EvaluationUploader;