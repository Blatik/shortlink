// API endpoint - change this after deploying Worker
const API_URL = 'https://url-shortener.blatik-short.workers.dev';

// DOM elements
const form = document.getElementById('shortenForm');
const urlInput = document.getElementById('urlInput');
const customAliasToggle = document.getElementById('customAliasToggle');
const customAliasInput = document.getElementById('customAlias');
const result = document.getElementById('result');
const shortUrlInput = document.getElementById('shortUrlInput');
const copyBtn = document.getElementById('copyBtn');
const btnText = document.querySelector('.btn-text');
const btnLoading = document.querySelector('.btn-loading');

// Toggle custom alias input
customAliasToggle.addEventListener('change', (e) => {
    customAliasInput.style.display = e.target.checked ? 'block' : 'none';
    if (!e.target.checked) {
        customAliasInput.value = '';
    }
});

// Handle form submission
form.addEventListener('submit', async (e) => {
    e.preventDefault();

    const url = urlInput.value.trim();
    const customAlias = customAliasToggle.checked ? customAliasInput.value.trim() : null;

    // Validate URL
    if (!isValidUrl(url)) {
        showError('Будь ласка, введіть правильний URL (http:// або https://)');
        return;
    }

    // Show loading state
    setLoading(true);

    try {
        // Call API
        const response = await fetch(`${API_URL}/api/shorten`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                url,
                custom_alias: customAlias || undefined,
            }),
        });

        const data = await response.json();

        if (!response.ok) {
            throw new Error(data.error || 'Помилка при створенні короткого посилання');
        }

        // Show result
        showResult(data.short_url);

    } catch (error) {
        showError(error.message);
    } finally {
        setLoading(false);
    }
});

// Copy to clipboard
copyBtn.addEventListener('click', async () => {
    const url = shortUrlInput.value;

    try {
        await navigator.clipboard.writeText(url);

        // Visual feedback
        const originalText = copyBtn.textContent;
        copyBtn.textContent = '✅ Скопійовано!';
        copyBtn.style.background = '#10b981';

        setTimeout(() => {
            copyBtn.textContent = originalText;
            copyBtn.style.background = '';
        }, 2000);

    } catch (error) {
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
