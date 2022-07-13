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

const root = ReactDOM.createRoot(
  document.getElementById('root') as HTMLElement
);
root.render(
  <ModelProvider>
    <React.StrictMode>
      <Router>
        <Routes>
          <Route index element={<FileUploader />} />
          <Route path="dashboard" element={<Dashboard />} />
          <Route path="*" element={<NotFound />} />
        </Routes>
      </Router>
    </React.StrictMode>
  </ModelProvider>
);
