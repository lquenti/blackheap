enum PreloadeeIOType {
    w = "w",
    r = "r"
}

type PreloadeeRecord = {
    io_type: PreloadeeIOType,
    bytes: number,
    sec: number,
}

export type {PreloadeeIOType, PreloadeeRecord};

type PreloadeeRecords = Array<PreloadeeRecord>;

export default PreloadeeRecords;