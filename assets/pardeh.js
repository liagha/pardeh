(() => {
    const apply = ({ region, html }) => {
        for (const host of document.querySelectorAll(`[data-pardeh="${region}"]`)) {
            host.innerHTML = html;
        }
    };
    const wire = () => {
        const source = new EventSource("/__pardeh/events");
        source.addEventListener("pd", (event) => apply(JSON.parse(event.data)));
        source.onerror = () => {
            source.close();
            setTimeout(() => location.reload(), 2000);
        };
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
    if (document.readyState === "loading") {
        addEventListener("DOMContentLoaded", wire);
    } else {
        wire();
    }
})();
