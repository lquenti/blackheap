import { useEffect, useState } from "react";
import PreloadeeRecords, {
  PreloadeeIOType,
  PreloadeeRecord,
} from "../../types/PreloadeeRecords";
import { parsePreloadeeData } from "../../utils/PreloadeeRecordsUtils";
import EvaluationDashboard from "./EvaluationDashboard";
import EvaluationUploader from "./EvaluationUploader";

// TODO: CREATE ONE SINGLE COMPONENT OUT
// OF BOTH UPLOADERS
const Evaluation = () => {
  const [preloadeeRecords, setPreloadeeRecords] =
    useState<PreloadeeRecords | null>(null);

  if (!preloadeeRecords) {
    return <EvaluationUploader setPreloadeeRecords={setPreloadeeRecords} />;
  }

  return <EvaluationDashboard preloadeeRecords={preloadeeRecords} />;
};
export default Evaluation;
