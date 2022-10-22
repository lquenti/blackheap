import { useContext, useEffect, useState } from "react";
import ModelContext from "../../contexts/ModelContext";
import Model, { BenchmarkType } from "../../types/Model";
import PreloadeeRecords, { ClassifiedPreloadeeRecords } from "../../types/PreloadeeRecords";
import { is_read_op_str } from "../../utils/ModelUtils";
import { classifyRecord } from "../../utils/PreloadeeRecordsUtils";

// TODO any
const EvaluationDashboard = ({preloadeeRecords}: {preloadeeRecords: PreloadeeRecords}) => {
    // Probably some headline
    // Then some table
    // afterwards some plots
    // but first, loading screen
    const model: Model = useContext(ModelContext)!.json;

    const [classifiedData, setClassifiedData] = useState<ClassifiedPreloadeeRecords | null>(null);

    useEffect(() => {
        // process once, but non-blockingly...
        setClassifiedData(preloadeeRecords.map(r => classifyRecord(model, r)));
        console.log("processing done");
    }, [])

    // Loading screen until the data is ready...
    if (!classifiedData) {
        return (
            <div className="grid h-screen place-items-center">
                <h1 className="text-3xl">Classifying Data...</h1>
            </div>
        );
    }
    
    const allClassificationTypes = [... new Set(model.map(m => m.benchmark_type))];    
    // each type has a read and write
    const allCombinations = allClassificationTypes.flatMap(type => [
        {type, is_read_op: false},
        {type, is_read_op: true}
    ]);

    return (
        // T
        <div className="mx-auto max-w-2xl">
            <h1 className="text-4xl underline my-5">
                Classification Report
            </h1>
            <p>{classifiedData.length} measurements classified.</p>
            <div className="overflow-x-auto m-9">
                <table className="table table-compact table-zebra w-full">
                    <thead>
                        <tr>
                            <th>Classification Type</th>
                            <th>Access Type</th>
                            <th>#Classified</th>
                            <th>Percentage</th>
                        </tr>
                    </thead>
                    <tbody>
                        {allCombinations.map(({type, is_read_op}, i) => {
                            const count = classifiedData
                                .filter(({predictedModel, preloadeeRecord}) =>
                                    predictedModel === type && (preloadeeRecord.io_type == "r") === is_read_op)
                                .length;
                            return (
                            <tr key={`classification-{i}`}>
                                <th>{type}</th>
                                <th>{is_read_op_str(is_read_op)}</th>
                                <th>{count}</th>
                                <th>{count / classifiedData.length * 100}%</th>
                            </tr>
                            );
                        }
                        )}
                    </tbody>
                </table>
            </div>
        </div>
    );
}

export default EvaluationDashboard;