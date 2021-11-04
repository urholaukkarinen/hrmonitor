<script>
    import {onMount} from 'svelte';
    import { Chart, registerables } from 'chart.js';

    const X_SCALE = 120;

    let values = new Array(X_SCALE).fill(0);
    let labels = [];

    let chart = null;
    let chartRef;
    onMount(async () => {
        Chart.register(...registerables);

        for (let i = 0; i < X_SCALE; i++) {
            labels.push(" ");
        }

        const {listen} = await import('@tauri-apps/api/event');

        await listen('current_hr', (e) => {
            let hr = e.payload;
            values.shift();
            values.push(hr);
            chart.update();
        });

        chart = new Chart(chartRef, {
            type: 'line',
            data: {
                labels: labels,
                datasets: [
                    {
                        label: 'Heart rate',
                        tension: 0.2,
                        borderColor: 'rgb(230, 25, 75)',
                        fill: false,
                        data: values,
                    }
                ],
            },
            options: {
                animations: {
                    numbers: {
                        properties: ["x"]
                    }
                },
                animation: {
                    duration: 500
                },
                elements: {
                    point:{
                        radius: 0
                    }
                },
                scales: {
                    y: {
                        min: 0,
                        max: 200,
                    },
                },
                plugins: {
                    legend: false
                }
            }
        });
    });
</script>

<canvas bind:this={chartRef}></canvas>