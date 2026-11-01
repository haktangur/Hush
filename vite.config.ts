import { defineConfig } from 'vitest/config';
import react from '@vitejs/plugin-react';
export default defineConfig({plugins:[react()],build:{target:'es2022'},clearScreen:false,server:{port:1420,strictPort:true},test:{include:['src/**/*.test.{ts,tsx}'],environment:'jsdom',setupFiles:['./tests/setup.ts']}});
