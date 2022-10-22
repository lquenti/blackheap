import { useContext, useEffect, useState } from "react";
import ModelContext from "../../contexts/ModelContext";
import Model, { BenchmarkType } from "../../types/Model";
import PreloadeeRecords, { ClassifiedPreloadeeRecords } from "../../types/PreloadeeRecords";

// TODO any
const EvaluationDashboard = ({preloadeeRecords}: {preloadeeRecords: PreloadeeRecords}) => {
    // Probably some headline
    // Then some table
    // afterwards some plots
    // but first, loading screen
    const model: Model = useContext(ModelContext)!.json;

    const [classifiedData, setClassifiedData] = useState<ClassifiedPreloadeeRecords | null>(null);

    // TODO MOVE ME OUT
    const classifyData = (preloadeeRecords: PreloadeeRecords): ClassifiedPreloadeeRecords => {
        return [
            {
                preloadeeRecord: preloadeeRecords[0],
                predictedModel: "RandomUncached" as BenchmarkType
            }
        ];
    } 

    useEffect(() => {
        // process once, but non-blockingly...
        setClassifiedData(classifyData(preloadeeRecords));
        console.log("processing done");
    }, [])

    if (!classifiedData) {
        return (
            <div className="grid h-screen place-items-center">
                <h1 className="text-3xl">Classifying Data...</h1>
            </div>
        );
    }
    return (
        // T
        <>
        <h1>{classifiedData[0].predictedModel}</h1>
        </>
    );
}

export default EvaluationDashboard;