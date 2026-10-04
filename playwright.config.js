import {defineConfig} from '@playwright/test';
export default defineConfig({
 testDir:'./tests/browser',timeout:60000,
 use:{baseURL:'http://127.0.0.1:8767',browserName:'chromium',launchOptions:process.env.TERRAIN_BROWSER_PATH?{executablePath:process.env.TERRAIN_BROWSER_PATH}:{}},
 webServer:{command:'python -m http.server 8767 --bind 127.0.0.1 --directory dist',url:'http://127.0.0.1:8767/TERRAIN/',reuseExistingServer:!process.env.CI},
});
