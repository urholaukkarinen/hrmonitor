<script>
	async function handleKeydown(event) {
		const { invoke } = await import('@tauri-apps/api/tauri');
		await invoke('toggle_decorations');
	}
</script>

<svelte:body />
<svelte:window on:dblclick={handleKeydown} />

<slot />

<div id="drag-overlay" data-tauri-drag-region />

<style>
	#drag-overlay {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		z-index: 99;
	}
</style>
