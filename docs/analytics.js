// Get short code from URL parameter
const urlParams = new URLSearchParams(window.location.search);
const shortCode = urlParams.get('code');

if (!shortCode) {
    window.location.href = 'index.html';
}

const API_URL = 'https://s.blatik-short.workers.dev';

// Display short link
document.getElementById('shortLinkDisplay').textContent = `${API_URL}/${shortCode}`;

// Fetch and render analytics
async function loadAnalytics() {
    try {
        const response = await fetch(`${API_URL}/api/analytics/${shortCode}`);
        if (!response.ok) {
            throw new Error('Failed to load analytics');
        }

        const data = await response.json();

        // Update stat cards
        document.getElementById('totalClicks').textContent = data.total_clicks || 0;

        if (data.countries && data.countries.length > 0) {
            document.getElementById('topCountry').textContent = data.countries[0].country || '-';
        }

        if (data.devices && data.devices.length > 0) {
            const topDevice = data.devices.reduce((a, b) => a.count > b.count ? a : b);
            document.getElementById('topDevice').textContent = topDevice.device_type || '-';
        }

        if (data.browsers && data.browsers.length > 0) {
            document.getElementById('topBrowser').textContent = data.browsers[0].browser || '-';
        }

        // Render charts
        renderTimelineChart(data.timeline || []);
        renderDeviceChart(data.devices || []);
        renderBrowserChart(data.browsers || []);
        renderCountryChart(data.countries || []);
        renderReferrersTable(data.referrers || []);

    } catch (error) {
        console.error('Error loading analytics:', error);
        alert('Failed to load analytics data');
    }
}

// Timeline Chart
function renderTimelineChart(timeline) {
    const ctx = document.getElementById('timelineChart').getContext('2d');

    new Chart(ctx, {
        type: 'line',
        data: {
            labels: timeline.map(t => t.date),
            datasets: [{
                label: 'Clicks',
                data: timeline.map(t => t.count),
                borderColor: '#8b5cf6',
                backgroundColor: 'rgba(139, 92, 246, 0.1)',
                fill: true,
                tension: 0.4
            }]
        },
        options: {
            responsive: true,
            maintainAspectRatio: true,
            plugins: {
                legend: {
                    display: false
                }
            },
            scales: {
                y: {
                    beginAtZero: true,
                    ticks: { color: '#94a3b8' },
                    grid: { color: 'rgba(255, 255, 255, 0.1)' }
                },
                x: {
                    ticks: { color: '#94a3b8' },
                    grid: { color: 'rgba(255, 255, 255, 0.1)' }
                }
            }
        }
    });
}

// Device Chart
function renderDeviceChart(devices) {
    const ctx = document.getElementById('deviceChart').getContext('2d');

    new Chart(ctx, {
        type: 'doughnut',
        data: {
            labels: devices.map(d => d.device_type),
            datasets: [{
                data: devices.map(d => d.count),
                backgroundColor: [
                    '#8b5cf6',
                    '#3b82f6',
                    '#10b981',
                    '#f59e0b'
                ]
            }]
        },
        options: {
            responsive: true,
            maintainAspectRatio: true,
            plugins: {
                legend: {
                    position: 'bottom',
                    labels: { color: '#94a3b8' }
                }
            }
        }
    });
}

// Browser Chart
function renderBrowserChart(browsers) {
    const ctx = document.getElementById('browserChart').getContext('2d');

    new Chart(ctx, {
        type: 'pie',
        data: {
            labels: browsers.map(b => b.browser),
            datasets: [{
                data: browsers.map(b => b.count),
                backgroundColor: [
                    '#8b5cf6',
                    '#3b82f6',
                    '#10b981',
                    '#f59e0b',
                    '#ef4444'
                ]
            }]
        },
        options: {
            responsive: true,
            maintainAspectRatio: true,
            plugins: {
                legend: {
                    position: 'bottom',
                    labels: { color: '#94a3b8' }
                }
            }
        }
    });
}

// Country Chart
function renderCountryChart(countries) {
    const ctx = document.getElementById('countryChart').getContext('2d');

    new Chart(ctx, {
        type: 'bar',
        data: {
            labels: countries.map(c => c.country),
            datasets: [{
                label: 'Clicks',
                data: countries.map(c => c.count),
                backgroundColor: '#8b5cf6'
            }]
        },
        options: {
            responsive: true,
            maintainAspectRatio: true,
            indexAxis: 'y',
            plugins: {
                legend: {
                    display: false
                }
            },
            scales: {
                x: {
                    beginAtZero: true,
                    ticks: { color: '#94a3b8' },
                    grid: { color: 'rgba(255, 255, 255, 0.1)' }
                },
                y: {
                    ticks: { color: '#94a3b8' },
                    grid: { color: 'rgba(255, 255, 255, 0.1)' }
                }
            }
        }
    });
}

// Referrers Table
function renderReferrersTable(referrers) {
    const container = document.getElementById('referrersTable');

    if (referrers.length === 0) {
        container.innerHTML = '<p class="empty-state">No referrer data available</p>';
        return;
    }

    container.innerHTML = referrers.map(r => `
        <div class="referrer-item">
            <span class="referrer-name">${r.referrer}</span>
            <span class="referrer-count">${r.count}</span>
        </div>
    `).join('');
}

// Load analytics on page load
loadAnalytics();
