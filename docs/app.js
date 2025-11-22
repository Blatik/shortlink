document.addEventListener('DOMContentLoaded', () => {
    console.log("App initialized");

    // API endpoint
    const API_URL = 'https://s.blatik-short.workers.dev';

    // DOM elements
    const form = document.getElementById('shortenForm');
    const urlInput = document.getElementById('urlInput');
    const customAliasToggle = document.getElementById('customAliasToggle');
    const customAliasInput = document.getElementById('customAlias');
    const resultDiv = document.getElementById('result');
    const shortUrlInput = document.getElementById('shortUrlInput');
    const copyBtn = document.getElementById('copyBtn');
    const submitBtn = form ? form.querySelector('button[type="submit"]') : null;
    const btnText = submitBtn ? submitBtn.querySelector('.btn-text') : null;
    const btnLoading = submitBtn ? submitBtn.querySelector('.btn-loading') : null;

    // Dashboard elements
    const dashboard = document.getElementById('dashboard');
    const linksList = document.getElementById('linksList');
    const emptyState = document.getElementById('emptyState');
    const refreshBtn = document.getElementById('refreshBtn');

    // User ID Management
    let googleToken = null;

    function getUserId() {
        // If signed in with Google, use the sub (Subject ID) from token
        if (googleToken) {
            const payload = parseJwt(googleToken);
            return payload.sub;
        }

        // Otherwise use local anonymous ID
        let userId = localStorage.getItem('user_id');
        if (!userId) {
            if (typeof crypto !== 'undefined' && crypto.randomUUID) {
                userId = crypto.randomUUID();
            } else {
                userId = 'user_' + Math.random().toString(36).substr(2, 9);
            }
            localStorage.setItem('user_id', userId);
        }
        return userId;
    }

    // Google Auth Callback (Global function for GSI)
    window.handleCredentialResponse = (response) => {
        console.log("Encoded JWT ID token: " + response.credential);
        googleToken = response.credential;

        const payload = parseJwt(googleToken);
        console.log("User:", payload);

        // Update UI
        document.querySelector('.g_id_signin').style.display = 'none';
        const userProfile = document.getElementById('userProfile');
        const userAvatar = document.getElementById('userAvatar');
        const userName = document.getElementById('userName');

        if (userProfile) {
            userProfile.style.display = 'flex';
            if (userAvatar) userAvatar.src = payload.picture;
            if (userName) userName.textContent = payload.name;
        }

        // Refresh dashboard for this user
        fetchUserLinks();
    };

    // Sign Out
    const signOutBtn = document.getElementById('signOutBtn');
    if (signOutBtn) {
        signOutBtn.addEventListener('click', () => {
            googleToken = null;
            document.querySelector('.g_id_signin').style.display = 'block';
            const userProfile = document.getElementById('userProfile');
            if (userProfile) userProfile.style.display = 'none';
            fetchUserLinks(); // Switch back to anonymous
        });
    }

    // JWT Parser
    function parseJwt(token) {
        try {
            var base64Url = token.split('.')[1];
            var base64 = base64Url.replace(/-/g, '+').replace(/_/g, '/');
            var jsonPayload = decodeURIComponent(window.atob(base64).split('').map(function (c) {
                return '%' + ('00' + c.charCodeAt(0).toString(16)).slice(-2);
            }).join(''));

            return JSON.parse(jsonPayload);
        } catch (e) {
            console.error("Error parsing JWT", e);
            return {};
        }
    }

    // Toggle custom alias input
    if (customAliasToggle && customAliasInput) {
        customAliasToggle.addEventListener('change', () => {
            customAliasInput.style.display = customAliasToggle.checked ? 'block' : 'none';
            if (customAliasToggle.checked) {
                customAliasInput.focus();
            }
        });
    }

    // Handle form submission
    if (form && urlInput && resultDiv && shortUrlInput) {
        form.addEventListener('submit', async (e) => {
            e.preventDefault();
            console.log("Form submitted");

            const url = urlInput.value.trim();
            const customAlias = customAliasToggle && customAliasInput && customAliasToggle.checked ? customAliasInput.value.trim() : null;

            if (!url) return;

            if (!isValidUrl(url)) {
                alert('Please enter a valid URL (http:// or https://)');
                return;
            }

            // Reset state
            setLoading(true);
            resultDiv.style.display = 'none';

            try {
                const headers = {
                    'Content-Type': 'application/json'
                };

                // Send appropriate ID
                if (googleToken) {
                    headers['Authorization'] = `Bearer ${googleToken}`;
                } else {
                    headers['X-User-ID'] = getUserId();
                }

                const response = await fetch(`${API_URL}/api/shorten`, {
                    method: 'POST',
                    headers: headers,
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
                console.error(error);
            } finally {
                setLoading(false);
            }
        });
    }

    // Copy to clipboard
    if (copyBtn && shortUrlInput) {
        copyBtn.addEventListener('click', () => {
            shortUrlInput.select();
            document.execCommand('copy');

            const originalText = copyBtn.innerText;
            copyBtn.innerText = '✅ Copied!';
            setTimeout(() => {
                copyBtn.innerText = originalText;
            }, 2000);
        });
    }

    // Loading state helper
    function setLoading(isLoading) {
        if (submitBtn) {
            submitBtn.disabled = isLoading;
            if (btnText) btnText.style.display = isLoading ? 'none' : 'inline';
            if (btnLoading) btnLoading.style.display = isLoading ? 'inline' : 'none';
        }
    }

    // Dashboard Functions
    async function fetchUserLinks() {
        try {
            const headers = {};
            if (googleToken) {
                headers['Authorization'] = `Bearer ${googleToken}`;
            } else {
                headers['X-User-ID'] = getUserId();
            }

            const response = await fetch(`${API_URL}/api/urls`, {
                headers: headers
            });

            if (!response.ok) return;

            const links = await response.json();
            renderLinks(links);
        } catch (error) {
            console.error('Error fetching links:', error);
        }
    }

    function renderLinks(links) {
        if (!linksList) return;
        linksList.innerHTML = '';

        if (links.length === 0) {
            if (dashboard) dashboard.style.display = 'block';
            if (emptyState) emptyState.style.display = 'block';
            return;
        }

        if (dashboard) dashboard.style.display = 'block';
        if (emptyState) emptyState.style.display = 'none';

        links.forEach(link => {
            const row = document.createElement('tr');
            const date = new Date(link.created_at).toLocaleDateString('en-US');

            const fullShortUrl = `${API_URL.replace('https://', '').split('/')[0]}/${link.short_code}`;

            row.innerHTML = `
                <td><a href="https://${fullShortUrl}" target="_blank" class="short-link">${link.short_code}</a></td>
                <td><span class="original-link" title="${link.original_url}">${link.original_url}</span></td>
                <td>${link.clicks}</td>
                <td>${date}</td>
                <td>
                    <button class="action-btn" onclick="copyLink('https://${fullShortUrl}')" title="Copy">📋</button>
                    <a href="analytics.html?code=${link.short_code}" class="action-btn" title="Analytics">📊</a>
                </td>
            `;
            linksList.appendChild(row);
        });
    }

    window.copyLink = (url) => {
        if (navigator.clipboard) { // Check if clipboard API is available
            navigator.clipboard.writeText(url);
            alert('Link copied!');
        } else {
            // Fallback for older browsers
            const tempInput = document.createElement('input');
            document.body.appendChild(tempInput);
            tempInput.value = url;
            tempInput.select();
            document.execCommand('copy');
            document.body.removeChild(tempInput);
            alert('Link copied!');
        }
    };

    if (refreshBtn) {
        refreshBtn.addEventListener('click', fetchUserLinks);
    }

    function isValidUrl(string) {
        try {
            const url = new URL(string);
            return url.protocol === 'http:' || url.protocol === 'https:';
        } catch {
            return false;
        }
    }

    // Initial load
    fetchUserLinks();
});
