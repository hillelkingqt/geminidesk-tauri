// Injected script to add top bar
(function() {
    // Wait for body
    const interval = setInterval(() => {
        if (document.body) {
            clearInterval(interval);
            injectTopBar();
        }
    }, 100);

    function injectTopBar() {
        // Create container for top bar
        const topBarContainer = document.createElement('div');
        topBarContainer.id = 'gemini-desk-top-bar';
        topBarContainer.style.position = 'fixed';
        topBarContainer.style.top = '0';
        topBarContainer.style.left = '0';
        topBarContainer.style.width = '100%';
        topBarContainer.style.zIndex = '99999';
        topBarContainer.style.height = '38px'; // Same as CSS

        // Push body down
        document.body.style.marginTop = '38px';

        // Use fetch to get content from local file (via custom protocol in Tauri if configured, but here we inline or fetch from resource)
        // Since we can't easily fetch local files in standard webview without custom protocol,
        // we will inline the HTML/CSS from drag.html here.

        const dragHtmlContent = `
<style>
        :root {
            --background-primary: #121212;
            --background-secondary: #1E1E1E;
            --text-primary: #E0E0E0;
            --text-secondary: #A0A0A0;
            --accent-glow: #00BFFF;
            --tooltip-background: #333333;
            --border-color: #333;
        }

        html.light-mode {
            --background-primary: #ffffff;
            --background-secondary: #ffffff;
            --text-primary: #212121;
            --text-secondary: #757575;
            --accent-glow: #007aff;
            --tooltip-background: #ffffff;
            --border-color: #e0e0e0;
        }

        .top-bar {
            width: 100%;
            height: 38px;
            -webkit-app-region: drag; /* This works in Tauri on Windows/macOS */
            background: var(--background-secondary);
            user-select: none;
            display: flex;
            justify-content: space-between;
            align-items: center;
            padding: 0 12px;
            border-bottom: 1px solid var(--border-color);
            box-shadow: 0 2px 10px rgba(0, 0, 0, 0.2);
            font-family: 'Poppins', sans-serif;
        }

        #chat-title {
            font-size: 14px;
            font-weight: 500;
            color: var(--text-secondary);
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
            max-width: calc(100% - 150px);
            -webkit-app-region: drag;
            transition: opacity 0.3s ease, transform 0.3s ease;
        }

        .button-container {
            display: flex;
            align-items: center;
            -webkit-app-region: no-drag;
        }

        .icon-button {
            -webkit-app-region: no-drag;
            width: 30px;
            height: 30px;
            border: none;
            background: none;
            color: var(--text-secondary);
            cursor: pointer;
            font-size: 20px;
            display: flex;
            align-items: center;
            justify-content: center;
            transition: color 0.3s ease, transform 0.4s cubic-bezier(0.175, 0.885, 0.32, 1.275);
            position: relative;
            margin-left: 6px;
        }

        .icon-button svg {
            width: 22px;
            height: 22px;
            transition: stroke 0.3s ease, transform 0.4s ease, fill 0.3s ease;
            stroke: var(--text-secondary);
            stroke-width: 1.5;
        }

        .icon-button:hover {
            transform: scale(1.1);
        }

        .icon-button:hover svg {
            stroke: var(--accent-glow);
            fill: var(--accent-glow);
            filter: drop-shadow(0 0 5px var(--accent-glow));
        }
    </style>
    <div class="top-bar">
        <span id="chat-title">GeminiDesk</span>
        <div class="button-container">
            <button id="minimize-button" class="icon-button" title="Minimize">
                <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path d="M5 12H19" stroke-linecap="round" stroke-linejoin="round" />
                </svg>
            </button>
            <button id="fullscreen-button" class="icon-button" title="Fullscreen">
                <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path d="M16 3h5v5M8 3H3v5M16 21h5v-5M8 21H3v-5" stroke-linecap="round" stroke-linejoin="round" />
                </svg>
            </button>
            <button id="close-window-button" class="icon-button" title="Close" style="display:flex;">
                <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path d="M6 6L18 18M6 18L18 6" stroke-linecap="round" stroke-linejoin="round" />
                </svg>
            </button>
        </div>
    </div>
        `;

        topBarContainer.innerHTML = dragHtmlContent;
        document.body.prepend(topBarContainer);

        // Attach event listeners
        document.getElementById('minimize-button').addEventListener('click', () => {
            window.__TAURI__.window.appWindow.minimize();
        });

        document.getElementById('fullscreen-button').addEventListener('click', () => {
            window.__TAURI__.window.appWindow.toggleMaximize();
        });

        document.getElementById('close-window-button').addEventListener('click', () => {
            window.__TAURI__.window.appWindow.close();
        });

        // Title updater
        setInterval(() => {
            const titleEl = document.querySelector('.conversation.selected .conversation-title') ||
                           document.querySelector('li.active a.prompt-link') ||
                           document.title;
            const displayTitle = (typeof titleEl === 'string' ? titleEl : titleEl?.textContent?.trim()) || 'GeminiDesk';
            document.getElementById('chat-title').textContent = displayTitle;
        }, 1000);
    }
})();
