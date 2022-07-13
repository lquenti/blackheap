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

const root = ReactDOM.createRoot(
  document.getElementById('root') as HTMLElement
);
root.render(
  <ModelProvider>
    <React.StrictMode>
      <Router>
        <Routes>
          <Route path="/" element={<FileUploader />} />
          <Route path="/dashboard" element={<Dashboard />} />
        </Routes>
      </Router>
    </React.StrictMode>
  </ModelProvider>
);
