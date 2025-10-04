import { defineConfig } from 'vite';

export default defineConfig({
  server: {
    watch: {
      usePolling: true,
    },
    host: '0.0.0.0', // ถ้าอยากเปิดจากเครื่องอื่นใน LAN ได้
    port: 5173
  }
});

