import PreloadeeRecords from "../../types/PreloadeeRecords";

// TODO any
const EvaluationDashboard = ({preloadeeRecords}: {preloadeeRecords: PreloadeeRecords}) => {
    return (
        <>
        {preloadeeRecords.map(r => r.io_type).join(",")}
        </>
    );
}

export default EvaluationDashboard;