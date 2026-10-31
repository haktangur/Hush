import { render, screen } from '@testing-library/react';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { expect, test, vi } from 'vitest';
import { App } from './App';
vi.mock('../infra/tauri/client',()=>({desktop:{bootstrap:vi.fn().mockRejectedValue(new Error('UNAVAILABLE'))}}));
test('bootstrap failure is visible without claiming storage succeeded',async()=>{render(<QueryClientProvider client={new QueryClient({defaultOptions:{queries:{retry:false}}})}><App/></QueryClientProvider>);expect(await screen.findByText('Oda açılamadı. Verilerin değiştirilmedi.')).toBeTruthy();expect(screen.queryByText('Yerel masaüstü bağlantısı hazır.')).not.toBeTruthy();});
