const CACHE_NAME = 'easyblue-v1';
const ASSETS_TO_CACHE = [
    '/',
    '/index.html',
    '/assets/style.css',
    '/assets/manifest.json',
    '/assets/icon-192.png',
    '/assets/icon-512.png',
    // Add your compiled .wasm and .js files here after your first build
];

// 1. Installation: Save all the files to the phone's memory
self.addEventListener('install', (event) => {
    event.waitUntil(
        caches.open(CACHE_NAME).then((cache) => {
            console.log('EasyBlue: Caching Shell Assets');
            return cache.addAll(ASSETS_TO_CACHE);
        })
    );
});

// 2. Activation: Clean up old versions of the app
self.addEventListener('activate', (event) => {
    event.waitUntil(
        caches.keys().then((keys) => {
            return Promise.all(
                keys.filter(key => key !== CACHE_NAME).map(key => caches.delete(key))
            );
        })
    );
});

// 3. Fetch: The "Offline Shield"
// If the phone is offline, it grabs the file from the Cache instead of the Web
self.addEventListener('fetch', (event) => {
    event.respondWith(
        caches.match(event.request).then((response) => {
            return response || fetch(event.request);
        })
    );
});

