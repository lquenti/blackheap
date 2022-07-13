import React from 'react';
import ReactDOM from 'react-dom/client';
import {
  HashRouter as Router,
  Routes,
  Route
} from "react-router-dom";

import './tailwind.css';

import {ModelProvider} from './contexts/ModelContext';
import FileUploader from './routes/FileUploader';
import Dashboard from './routes/Dashboard';
import NotFound from './routes/NotFound';

import RandomUncachedRead from './routes/RandomUncachedRead';
import RandomUncachedWrite from './routes/RandomUncachedWrite';
import SameOffsetRead from './routes/SameOffsetRead';
import SameOffsetWrite from './routes/SameOffsetWrite';
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
            <Route path="randomread" element={<RandomUncachedRead />} />
            <Route path="randomwrite" element={<RandomUncachedWrite />} />
            <Route path="offsetread" element={<SameOffsetRead />} />
            <Route path="offsetwrite" element={<SameOffsetWrite />} />
            <Route path="evaluation" element={<Evaluation />} />
            <Route path="*" element={<NotFound />} />
          </Route>
          <Route path="*" element={<NotFound />} />
        </Routes>
      </Router>
    </React.StrictMode>
  </ModelProvider>
);
