import { useContext, useEffect, useState } from "react";
import StackedHistogram from "../../components/StackedHistogram";
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

    const [lowerSelection, setLowerSelection] = useState<number>(0);
    const [upperSelection, setUpperSelection] = useState<number>(Number.POSITIVE_INFINITY);

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

    const updateView = () => {
        const lower = parseInt((document.getElementById("lower") as HTMLInputElement).value) || 0;
        const upper = parseInt((document.getElementById("upper") as HTMLInputElement).value) || Number.POSITIVE_INFINITY;
        setLowerSelection(lower);
        setUpperSelection(upper);


    }

    return (
        // T
        <div className="mx-auto max-w-2xl">
            <h1 className="text-4xl underline mt-9 mb-3">
                Classification Report
            </h1>
            <p>{classifiedData.length} measurements classified.</p>
            <h2 className="text-3xl underline">All Measurements</h2>
            <StackedHistogram allCombinations={allCombinations} classifiedData={classifiedData} />
            <h2 className="text-3xl underline">Select Access Sizes to View:</h2>
            <div>
                <div className="flex items-center mx-auto shadow rounded border-0 py-3">
                    <input type="number" className="flex-grow input bg-neutral input-bordered mr-3" id="lower" placeholder="Lower Bound" />
                    <input type="number" className="flex-grow input bg-neutral input-bordered mx-3" id="upper" placeholder="Upper Bound" />
                    <button className="btn btn-primary ml-3" onClick={updateView}>Update</button>
                </div>
            </div>

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
                            const all = classifiedData
                                .filter(({preloadeeRecord}) =>
                                        preloadeeRecord.bytes >= lowerSelection &&
                                        preloadeeRecord.bytes <= upperSelection
                                );
                                const count = all
                                .filter(({predictedModel, preloadeeRecord}) =>
                                        predictedModel === type &&
                                        (preloadeeRecord.io_type == "r") === is_read_op
                                )
                                .length;
                            return (
                            <tr key={`classification-${i}`}>
                                <th>{type}</th>
                                <th>{is_read_op_str(is_read_op)}</th>
                                <th>{count}</th>
                                <th>{(count / all.length * 100).toFixed(2)}%</th>
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