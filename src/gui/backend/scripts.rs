pub fn get_full_init_script() -> String {
    r#"
    window.__IPC_CALLBACK__ = function(response) {
        const event = new CustomEvent('ipc-response', { 
            detail: response 
        });
        window.dispatchEvent(event);
    };

    window._ipc_ = {
        send: function(type, payload) {
            const id = Math.random().toString(36).substr(2, 9);
            window.ipc.postMessage(JSON.stringify({
                id,
                type,
                payload
            }));
            return id;
        }
    };
    "#.to_string()
}
