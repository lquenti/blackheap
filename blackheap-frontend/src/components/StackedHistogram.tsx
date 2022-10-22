import Plot from "react-plotly.js";
import { BenchmarkType } from "../types/Model";
import { ClassifiedPreloadeeRecords } from "../types/PreloadeeRecords";

type Parameters = {
  allCombinations: {
    type: BenchmarkType;
    is_read_op: boolean;
  }[];
  classifiedData: ClassifiedPreloadeeRecords | null;
};

function StackedHistogram({ allCombinations, classifiedData }: Parameters) {
  /*
    const data: Plotly.data = [...allCombinations.map(({type, is_read_op}) =>
        {
            return {
                x: classifiedData?.filter(({predictedModel, preloadeeRecord}) => 
                    predictedModel === type &&
                    (preloadeeRecord.io_type == "r") === is_read_op
                ).map(({predictedModel, preloadeeRecord}) => preloadeeRecord.bytes) as NonNullable<number[]>,
                type: "histogram"
            }
        }
    )];
    */
  return <div></div>;
}

export default StackedHistogram;
