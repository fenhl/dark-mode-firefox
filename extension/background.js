var port = browser.runtime.connectNative('dark_mode');
port.onMessage.addListener((message) => {
    if (message) {
        // switch to dark theme
        browser.theme.update({
            "colors": {
                "tab_background_text": "rgb(249, 249, 250)",
                "icons": "rgb(249, 249, 250, 0.7)",
                "frame": "hsl(240, 5%, 5%)",
                "popup": "#4a4a4f",
                "popup_text": "rgb(249, 249, 250)",
                "popup_border": "#27272b",
                "tab_line": "#0a84ff",
                "toolbar": "hsl(240, 1%, 20%)",
                "toolbar_bottom_separator": "hsl(240, 5%, 5%)",
                "toolbar_field": "rgb(71, 71, 73)",
                "toolbar_field_border": "rgba(249, 249, 250, 0.2)",
                "toolbar_field_separator": "#5F6670",
                "toolbar_field_text": "rgb(249, 249, 250)",
                "ntp_background": "#2A2A2E",
                "ntp_text": "rgb(249, 249, 250)",
                "sidebar": "#38383D",
                "sidebar_text": "rgb(249, 249, 250)",
                "sidebar_border": "rgba(255, 255, 255, 0.1)"
            }
        })
    } else {
        // switch to light theme
        browser.theme.update({
            "colors": {
                "tab_background_text": "rgb(24, 25, 26)",
                "icons": "rgb(24, 25, 26, 0.7)",
                "frame": "#E3E4E6",
                "popup": "#fff",
                "popup_text": "#0c0c0d",
                "popup_border": "#ccc",
                "tab_line": "#0a84ff",
                "toolbar": "#f5f6f7",
                "toolbar_bottom_separator": "#ccc",
                "toolbar_field": "#fff",
                "toolbar_field_border": "#ccc"
            }
        })
    }
});
