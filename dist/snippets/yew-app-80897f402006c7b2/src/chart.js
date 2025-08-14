let chart = null;

export function draw_chart(labelsArray, dataArray) {
    const ctx = document.getElementById('growthChart');
    
    if (!ctx) {
        console.error('Canvas element with id "growthChart" not found');
        return;
    }

    // Destroy existing chart if it exists
    if (chart) {
        chart.destroy();
    }

    chart = new Chart(ctx, {
        type: 'line',
        data: {
            labels: labelsArray,
            datasets: [{
                label: 'Portfolio Value',
                data: dataArray,
                borderColor: 'rgb(75, 192, 192)',
                backgroundColor: 'rgba(75, 192, 192, 0.2)',
                tension: 0.1,
                fill: true
            }]
        },
        options: {
            responsive: true,
            maintainAspectRatio: false,
            scales: {
                y: {
                    beginAtZero: false,
                    ticks: {
                        callback: function(value) {
                            return '$' + value.toLocaleString();
                        }
                    }
                },
                x: {
                    title: {
                        display: true,
                        text: 'Months'
                    }
                }
            },
            plugins: {
                tooltip: {
                    callbacks: {
                        label: function(context) {
                            return 'Portfolio Value: $' + context.parsed.y.toLocaleString();
                        }
                    }
                },
                legend: {
                    display: true,
                    position: 'top'
                }
            }
        }
    });
}