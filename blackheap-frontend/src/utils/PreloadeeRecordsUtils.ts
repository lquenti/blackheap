// TODO remove utils from all utils names

import PreloadeeRecords, { PreloadeeIOType, PreloadeeRecord } from "../types/PreloadeeRecords";

const parsePreloadee = (recordStr: string): PreloadeeRecord => {
    const [io_type_str, bytes_str, sec_str] = recordStr.trim().split(',');
    const io_type = io_type_str as PreloadeeIOType;
    const bytes = parseInt(bytes_str);
    const sec = parseFloat(sec_str);
    return {
        io_type,
        bytes,
        sec
    }
}

// first == csv col declaration
const parsePreloadees = (csvStr: string): PreloadeeRecords => 
    csvStr.trim().split(',').slice(1).map(parsePreloadee)

export {parsePreloadees}