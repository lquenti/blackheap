import { BenchmarkType } from "./Model";

enum PreloadeeIOType {
  w = "w",
  r = "r",
}

type PreloadeeRecord = {
  io_type: PreloadeeIOType;
  bytes: number;
  sec: number;
};

type ClassifiedPreloadeeRecord = {
  preloadeeRecord: PreloadeeRecord,
  predictedModel: BenchmarkType
};

type ClassifiedPreloadeeRecords = Array<ClassifiedPreloadeeRecord>;

export type { ClassifiedPreloadeeRecords, PreloadeeIOType, PreloadeeRecord };

type PreloadeeRecords = Array<PreloadeeRecord>;

export default PreloadeeRecords;
