if(import.meta.env.MODE==='e2e'){await import('@wdio/tauri-plugin')}
import React from 'react';
import ReactDOM from 'react-dom/client';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { App } from './app/App';
import { installDateEmptyState } from './app/dateInputs';
import './styles/theme.css';
import './styles/global.css';
const queryClient = new QueryClient({defaultOptions:{queries:{retry:false}}});
ReactDOM.createRoot(document.getElementById('root')!).render(<React.StrictMode><QueryClientProvider client={queryClient}><App/></QueryClientProvider></React.StrictMode>);
installDateEmptyState();
