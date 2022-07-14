import React from 'react';
import ReactDOM from 'react-dom/client';
import {
  HashRouter as Router,
  Routes,
  Route
} from "react-router-dom";

import './tailwind.css';

import { ModelProvider } from './contexts/ModelContext';
import FileUploader from './routes/FileUploader';
import Dashboard from './routes/Dashboard';
import NotFound from './routes/NotFound';

import PlotView from './routes/Plotview'; import { BenchmarkType } from './types/Model';
import Overview from './routes/Overview';
import Evaluation from './routes/Evaluation';

const root = ReactDOM.createRoot(
  document.getElementById('root') as HTMLElement
);
root.render(
  <ModelProvider>
    <React.StrictMode>
      <Router>
        <Routes>
          <Route index element={<FileUploader />} />
          <Route path="dashboard" element={<Dashboard />}>
            <Route path="overview" element={<Overview />} />
            <Route path="randomread" element={<PlotView benchmark_type={BenchmarkType.RandomUncached} is_read_op={true} />} />
            <Route path="randomwrite" element={<PlotView benchmark_type={BenchmarkType.RandomUncached} is_read_op={false} />} />
            <Route path="offsetread" element={<PlotView benchmark_type={BenchmarkType.SameOffset} is_read_op={true} />} />
            <Route path="offsetwrite" element={<PlotView benchmark_type={BenchmarkType.SameOffset} is_read_op={false} />} />
            <Route path="evaluation" element={<Evaluation />} />
            <Route path="*" element={<NotFound />} />
          </Route>
          <Route path="*" element={<NotFound />} />
        </Routes>
      </Router>
    </React.StrictMode>
  </ModelProvider>
);
