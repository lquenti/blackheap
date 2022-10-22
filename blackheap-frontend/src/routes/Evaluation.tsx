

// TODO: CREATE ONE SINGLE COMPONENT OUT

import { useEffect, useState } from "react";
import { useFilePicker } from "use-file-picker";
import PreloadeeRecords, {PreloadeeIOType, PreloadeeRecord} from "../types/PreloadeeRecords";

// OF BOTH UPLOADERS
const Evaluation = () => {
  const [openFileSelector, { filesContent }] = useFilePicker({
    accept: ".csv",
    limitFilesConfig: {max: 1},
  });
  const [enableRedirect, setEnableRedirect] = useState(false);
  
  // TODO: Move outside of here into a helper folder
  const parsePreloadeeData = (unparsed: string) => {
    const arr: Array<string> = unparsed.split("\n");

    // First line is the csv header which we can skip
    // should look sth along the lines of
    // io_type,bytes,sec
    arr.shift();
    return arr
      .filter(s => s !== '')
      .map(parseSingleLine);
  }

  // TODO: Move outside as well
  const parseSingleLine = (line: string): PreloadeeRecord => {
    // A line should look like
    // r,704,1.2119999155402184e-06
    const [typeStr, bytesStr, secStr] = line.split(",");
    const io_type: PreloadeeIOType = typeStr as PreloadeeIOType;
    return {
      io_type,
      bytes: parseInt(bytesStr),
      sec: parseFloat(secStr)
    };
  }

  useEffect(() => {
    if (filesContent.length !== 0) {
      const unparsedCsv: string = filesContent[0].content;
      const parsed: PreloadeeRecords = parsePreloadeeData(unparsedCsv);
      console.log("b4");
      console.log(parsed);
      console.log("after");
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
