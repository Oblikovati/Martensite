import init, { run_all_e2e_checks } from "./pkg/martensite_wasm_e2e.js";

self.onmessage = async (ev) => {
    try {
        const { canvas, maxWorkers } = ev.data;
        await init();
        const glueUrl = new URL("./pkg/martensite_wasm_e2e.js", import.meta.url).href;
        const json = await run_all_e2e_checks(canvas, maxWorkers, glueUrl);
        self.postMessage({ ok: true, json });
    } catch (err) {
        self.postMessage({ ok: false, error: String(err?.stack || err) });
    }
};
