const { app, BrowserWindow } = require('electron');

function createWindow() {
    let win = new BrowserWindow({
        fullscreen: true,
        webPreferences: {
            nodeIntegration: true,
        },
    });

    win.loadFile('dist/index.html');

    win = null;
}

app.whenReady().then(createWindow);

app.on('window-all-closed', () => {
    if (process.platform != 'darwin') {
        app.quit();
    }
});

app.on('activate', () => {
    if (win === null) {
        createWindow();
    }
});