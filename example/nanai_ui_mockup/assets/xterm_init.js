// xterm_init.js
// グローバル関数として登録し、wasm_bindgenからも確実に呼び出せるようにする

globalThis.init_xterm = (id) => {
	console.log("init_xterm called", id);
	// CDNからxterm.jsをグローバルロード
	if (!globalThis.Terminal) {
		const script = document.createElement("script");
		script.src = "https://cdn.jsdelivr.net/npm/xterm@5.3.0/lib/xterm.js";
		script.onload = () => {
			console.log("xterm.js loaded");
			const style = document.createElement("link");
			style.rel = "stylesheet";
			style.href = "https://cdn.jsdelivr.net/npm/xterm@5.3.0/css/xterm.css";
			document.head.appendChild(style);

			const term = new globalThis.Terminal();
			term.open(document.getElementById(id));
			term.write("Welcome to xterm.js\r\n");
			globalThis._xterm = term;
		};
		document.head.appendChild(script);
	} else {
		console.log("xterm.js already loaded");
		const term = new globalThis.Terminal();
		term.open(document.getElementById(id));
		term.write("Welcome to xterm.js\r\n");
		globalThis._xterm = term;
	}
};
