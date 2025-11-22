// API endpoint
const API_URL = 'https://url-shortener.blatik-short.workers.dev';

// DOM elements
const form = document.getElementById('shortenForm');
const urlInput = document.getElementById('urlInput');
const customAliasToggle = document.getElementById('customAliasToggle');
const customAliasInput = document.getElementById('customAlias');
const resultDiv = document.getElementById('result');
const shortUrlInput = document.getElementById('shortUrlInput');
const copyBtn = document.getElementById('copyBtn');
const submitBtn = form.querySelector('button[type="submit"]');
const btnText = submitBtn.querySelector('.btn-text');
const btnLoading = submitBtn.querySelector('.btn-loading');

// Dashboard elements
const dashboard = document.getElementById('dashboard');
const linksList = document.getElementById('linksList');
const emptyState = document.getElementById('emptyState');
const refreshBtn = document.getElementById('refreshBtn');

// User ID Management
function getUserId() {
    let userId = localStorage.getItem('user_id');
    if (!userId) {
        userId = crypto.randomUUID();
        localStorage.setItem('user_id', userId);
    }
    return userId;
}

const USER_ID = getUserId();

// Toggle custom alias input
customAliasToggle.addEventListener('change', () => {
    customAliasInput.style.display = customAliasToggle.checked ? 'block' : 'none';
    if (customAliasToggle.checked) {
        customAliasInput.focus();
    }
});

// Handle form submission
form.addEventListener('submit', async (e) => {
    e.preventDefault();

    const url = urlInput.value.trim();
    const customAlias = customAliasToggle.checked ? customAliasInput.value.trim() : null;

    if (!url) return;

    // Reset state
    setLoading(true);
    resultDiv.style.display = 'none';

    try {
        const response = await fetch(`${API_URL}/api/shorten`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'X-User-ID': USER_ID
            },
            body: JSON.stringify({
                url: url,
                custom_alias: customAlias
            })
        });

        const data = await response.json();

        if (!response.ok) {
            throw new Error(data.error || 'Something went wrong');
        }

        // Show result
        shortUrlInput.value = data.short_url;
        resultDiv.style.display = 'block';

        // Refresh dashboard
        fetchUserLinks();

    } catch (error) {
        alert(error.message);
    } finally {
        setLoading(false);
    }
});

// Copy to clipboard
copyBtn.addEventListener('click', () => {
    shortUrlInput.select();
    document.execCommand('copy');

    const originalText = copyBtn.innerText;
    copyBtn.innerText = '✅ Скопійовано!';
    setTimeout(() => {
        copyBtn.innerText = originalText;
    }, 2000);
});

// Loading state helper
function setLoading(isLoading) {
    submitBtn.disabled = isLoading;
    btnText.style.display = isLoading ? 'none' : 'inline';
    btnLoading.style.display = isLoading ? 'inline' : 'none';
}

// Dashboard Functions
async function fetchUserLinks() {
    try {
        const response = await fetch(`${API_URL}/api/urls`, {
            headers: {
                'X-User-ID': USER_ID
            }
        });

        if (!response.ok) return;

        const links = await response.json();
        renderLinks(links);
    } catch (error) {
        console.error('Error fetching links:', error);
    }
}

function renderLinks(links) {
    linksList.innerHTML = '';

    if (links.length === 0) {
        dashboard.style.display = 'none'; // Hide dashboard if no links (optional, or show empty state)
        // Actually, let's show dashboard but with empty state if user has visited before?
        // For now, show if links exist.
        return;
    }

    dashboard.style.display = 'block';
    emptyState.style.display = 'none';

    links.forEach(link => {
        const row = document.createElement('tr');
        const date = new Date(link.created_at).toLocaleDateString('uk-UA');
        const shortUrl = `${API_URL.replace('/api', '')}/${link.short_code}`; // Construct URL roughly or use base_url if returned

        // Use base URL from env if possible, but here we construct it
        // The API returns short_url in shorten response but not in list (yet).
        // Let's assume the worker URL is the base.
        const fullShortUrl = `${API_URL.replace('https://', '').split('/')[0]}/${link.short_code}`;

        row.innerHTML = `
            <td><a href="https://${fullShortUrl}" target="_blank" class="short-link">${link.short_code}</a></td>
            <td><span class="original-link" title="${link.original_url}">${link.original_url}</span></td>
            <td>${link.clicks}</td>
            <td>${date}</td>
            <td>
                <button class="action-btn" onclick="copyLink('https://${fullShortUrl}')" title="Копіювати">📋</button>
            </td>
        `;
        linksList.appendChild(row);
    });
}

window.copyLink = (url) => {
    navigator.clipboard.writeText(url);
    alert('Посилання скопійовано!');
};

refreshBtn.addEventListener('click', fetchUserLinks);

// Initial load
fetchUserLinks();
setTimeout(() => {
    copyBtn.textContent = originalText;
    copyBtn.style.background = '';
    // Fallback for older browsers
    shortUrlInput.select();
    document.execCommand('copy');
    alert('Посилання скопійовано!');
}
});

// Helper functions
function isValidUrl(string) {
    try {
        const url = new URL(string);
        return url.protocol === 'http:' || url.protocol === 'https:';
    } catch {
        return false;
    }
}

function showResult(shortUrl) {
    shortUrlInput.value = shortUrl;
    result.style.display = 'block';

    // Smooth scroll to result
    result.scrollIntoView({ behavior: 'smooth', block: 'nearest' });
}

function showError(message) {
    alert('❌ ' + message);
}

function setLoading(loading) {
    if (loading) {
        btnText.style.display = 'none';
        btnLoading.style.display = 'inline';
        form.querySelector('button[type="submit"]').disabled = true;
    } else {
        btnText.style.display = 'inline';
        btnLoading.style.display = 'none';
        form.querySelector('button[type="submit"]').disabled = false;
    }
}

// Auto-focus on URL input
urlInput.focus();
