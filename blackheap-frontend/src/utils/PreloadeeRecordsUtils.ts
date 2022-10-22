import Model, { BenchmarkType } from "../types/Model";
import { PreloadeeRecord, PreloadeeIOType, ClassifiedPreloadeeRecords, ClassifiedPreloadeeRecord } from "../types/PreloadeeRecords";
import { evaluate } from "./ModelUtils";

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

const classifyRecord = (model: Model, preloadeeRecord: PreloadeeRecord): ClassifiedPreloadeeRecord => {
  const {io_type, bytes, sec} = preloadeeRecord;
  const isReadOp = io_type === "r";
  
  // Initialization of the return types
  let predictedModel = "Unclassified" as BenchmarkType;
  let upperBound = Number.POSITIVE_INFINITY;

  for (const analysis of model) {
    // we only want to compare those of the same io_type (apples and oranges...)
    if (analysis.is_read_op !== isReadOp) {
      continue;
    }

    // if it took less than our expected time we can disquality it
    // (something is classified as X iff X expected value is the lowest upper bound)
    //
    // TODO: WE CURRENTLY HAVE A LOT OF ROUNDING ERRORS TO ZERO (THANKS JS)
    // THUS, FOR NOW THAT WE DONT GET FALSE RESULTS, IT HAS TO BE SMALLER EQUAL
    // ONCE THATS FIXED, CHANGE ME TO STRICTLY SMALLER
    const expectedTime = evaluate(analysis.model, bytes);
    if (expectedTime <= sec) {
      continue;
    }

    // if it is a tighter lower bound, it is a better classification
    if (expectedTime < upperBound) {
      predictedModel = analysis.benchmark_type;
      upperBound = expectedTime;
    }
  }

  return {
    preloadeeRecord,
    predictedModel,
  };
}

export {classifyRecord, parsePreloadeeData, parseSingleLine};