<script>
	import { onMount } from 'svelte';

	let current_hr = 0;
    let max_hr = 0;
    let count = 0;

	onMount(async () => {
        const { listen } = await import('@tauri-apps/api/event');

        await listen('current_hr', (e) => {
            current_hr = e.payload;
            count += 1;
            max_hr = Math.max(current_hr, max_hr);
        });
	});
</script>

<style>
    #hr {
        display: inline-block;
    }

    #current-hr {
        font-family: "Droid Sans", sans-serif;
        color: rgb(230, 25, 75);
        font-size: 100px;
        display: grid;
        float: left;
    }

    #max-hr {
        font-family: "Droid Sans", sans-serif;
        color: white;
        font-size: 40px;
        display: grid;
        margin-left: 20px;
    }
</style>

<div id="hr">
    <div id="current-hr">{current_hr}</div>
    <div id="max-hr">{max_hr}</div>
</div>

