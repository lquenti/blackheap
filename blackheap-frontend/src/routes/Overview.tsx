import { useContext } from 'react';
import Formula from '../components/Formula';
import ModelContext from '../contexts/ModelContext';
import Model from '../types/Model';
import UnifiedFunctionPlot from '../components/UnifiedFunctionPlot';
import { benchmark_type_str, equation_str, is_read_op_str } from '../utils/ModelUtils';

// TODO: Reduce redundancy with PlotView
const Overview = () => {
    const model: Model = useContext(ModelContext)!.json;
    return (
        <div className="mx-auto max-w-2xl">
            <h1 className="text-center text-4xl mt-3 mb-9">Overview:</h1>
            {/* Formulas */}
            {model.map((m, i) => (
                <div key={`formula-${i}`} className="py-5">
                    <h2 className="text-2xl">{benchmark_type_str(m.benchmark_type)}: {is_read_op_str(m.is_read_op)}</h2>
                    <div className="pt-3">
                        <Formula tex={equation_str(m.model)} />
                    </div>
                </div>
            ))}
            {/* Unified Function Plot */}
            <div>
                <UnifiedFunctionPlot />
            </div>
        </div>
    )
};

export default Overview;
