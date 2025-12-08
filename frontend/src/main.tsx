
// どこからも参照しないのにインポートしないといけない罠がある
import "./index.css";

import React from 'react';
import ReactDOM from 'react-dom/client';
import { BrowserRouter } from 'react-router-dom';
import { QueryProvider } from './app/providers/query-client';
import { ThemeProvider } from './app/providers/theme-provider';
import { AppRouter } from './app/router';

const rootElement = document.getElementById('root');

if (!rootElement) {
  throw new Error('Failed to find the root element');
}

ReactDOM.createRoot(rootElement).render(
  <React.StrictMode>
    <QueryProvider>
      <ThemeProvider defaultTheme="system" storageKey="ui-theme">
        <BrowserRouter>
          <AppRouter />
        </BrowserRouter>
      </ThemeProvider>
    </QueryProvider>
  </React.StrictMode>
)