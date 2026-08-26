(() => {
    const apply = ({ region, html }) => {
        for (const host of document.querySelectorAll(`[data-pardeh="${region}"]`)) {
            host.innerHTML = html;
        }
    };
    const watchForms = () => {
        document.addEventListener("submit", (event) => {
            const form = event.target;
            if (!form.closest("[data-pardeh]")) return;
            event.preventDefault();
            fetch(form.action, {
                method: (form.method || "post").toUpperCase(),
                body: new FormData(form),
            }).catch(() => location.reload());
        });
    };
    const wire = () => {
        watchForms();
        if (!document.querySelector("[data-pardeh]")) return;
        let seen = false;
        let tries = 0;
        const connect = () => {
            const source = new EventSource("/__pardeh/events");
            source.addEventListener("pd", (event) => apply(JSON.parse(event.data)));
            source.onopen = () => {
                seen = true;
                tries = 0;
            };
            source.onerror = () => {
                source.close();
                if (seen) {
                    location.reload();
                    return;
                }
                setTimeout(connect, Math.min(1000 * 2 ** tries, 8000));
                tries += 1;
            };
        };
        connect();
    };
    if (document.readyState === "loading") {
        addEventListener("DOMContentLoaded", wire);
    } else {
        wire();
    }
})();
