// Mock Electron API for Tauri
(function() {
    // Listen for events
    if (window.__TAURI__) {
        const { listen } = window.__TAURI__.event;
        listen('theme-updated', (event) => {
            // theme logic here
        });
        listen('settings-updated', (event) => {
            // settings logic
        });
    }

    window.electronAPI = {
        updateSetting: (key, value) => {
            if (window.__TAURI__) {
                window.__TAURI__.invoke('update_setting', { key, value });
            }
        },
        getSettings: () => {
            if (window.__TAURI__) {
                return window.__TAURI__.invoke('get_settings');
            }
            return Promise.resolve({});
        },
        onSettingsUpdated: (callback) => {
            if (window.__TAURI__) {
                window.__TAURI__.event.listen('settings-updated', (event) => {
                    callback(event.payload);
                });
            }
        },
        theme: {
            getResolved: () => Promise.resolve('dark'), // Can invoke rust if needed
            onUpdate: (callback) => {
                if (window.__TAURI__) {
                    window.__TAURI__.event.listen('theme-updated', (event) => {
                        callback(event.payload);
                    });
                }
            }
        },
        toggleFullScreen: () => {
            if (window.__TAURI__) window.__TAURI__.invoke('toggle_maximize');
        },
        openSettingsWindow: () => console.log('Open Settings'),
        openNewWindow: () => window.open('https://gemini.google.com/app', '_blank'),
        minimizeWindow: () => {
            if (window.__TAURI__) window.__TAURI__.invoke('minimize_window');
        },
        exportChat: () => console.log('Export Chat'),
        toggleAppMode: (mode) => {
            if (window.__TAURI__) window.__TAURI__.invoke('update_setting', { key: 'appMode', value: mode });
            if (mode === 'aistudio') {
                window.location.href = 'https://aistudio.google.com';
            } else {
                window.location.href = 'https://gemini.google.com/app';
            }
        },
        getAppMode: () => {
            if (window.__TAURI__) return window.__TAURI__.invoke('get_app_mode');
            return Promise.resolve('gemini');
        },
        onAppModeChanged: (callback) => {},
        executeInMainView: (code) => {
            try {
                // Dangerous but matches previous capabilities
                return Promise.resolve(eval(code));
            } catch (e) {
                return Promise.reject(e);
            }
        },
        onLanguageChanged: (callback) => {
             if (window.__TAURI__) {
                window.__TAURI__.event.listen('language-changed', (event) => {
                    callback(event.payload);
                });
            }
        }
    };

    window.chatAPI = {
        onTitleUpdate: (callback) => {
            setInterval(() => {
                const titleEl = document.querySelector('.conversation.selected .conversation-title') ||
                               document.querySelector('li.active a.prompt-link') ||
                               document.querySelector('[data-test-id="conversation-title"]');
                const title = titleEl ? titleEl.textContent.trim() : 'New Chat';
                callback(title);
            }, 1000);
        }
    };
})();
