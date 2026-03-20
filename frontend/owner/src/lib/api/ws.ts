/**
 * Creates a reconnecting WebSocket connection for real-time queue updates.
 */
export function createQueueSocket(locationId: string, onUpdate: () => void): { close: () => void } {
	let ws: WebSocket | null = null;
	let closed = false;
	let reconnectTimer: ReturnType<typeof setTimeout>;

	function connect() {
		if (closed) return;

		const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
		const url = `${protocol}//${window.location.host}/api/v1/queue/ws/${locationId}`;

		ws = new WebSocket(url);

		ws.onmessage = (event) => {
			if (event.data === 'queue_updated') {
				onUpdate();
			}
		};

		ws.onclose = () => {
			if (!closed) {
				reconnectTimer = setTimeout(connect, 3000);
			}
		};

		ws.onerror = () => {
			ws?.close();
		};
	}

	connect();

	return {
		close() {
			closed = true;
			clearTimeout(reconnectTimer);
			ws?.close();
		}
	};
}
