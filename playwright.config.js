import {defineConfig} from '@playwright/test';
const live=process.env.TERRAIN_SITE_URL;
export default defineConfig({
 testDir:'./tests/browser',timeout:60000,
 use:{baseURL:live||'http://127.0.0.1:8767',browserName:'chromium',launchOptions:process.env.TERRAIN_BROWSER_PATH?{executablePath:process.env.TERRAIN_BROWSER_PATH}:{}},
 webServer:live?undefined:{command:'python tools/serve-pages.py',url:'http://127.0.0.1:8767/TERRAIN/',reuseExistingServer:false},
});
