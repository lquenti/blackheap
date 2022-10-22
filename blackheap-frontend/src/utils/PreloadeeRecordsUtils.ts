import { PreloadeeRecord, PreloadeeIOType } from "../types/PreloadeeRecords";

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

export {parsePreloadeeData, parseSingleLine};