async function fetchAuth(url, options = {}) {

    const token = getToken();

    options.headers = {
        ...options.headers,
        "Authorization": "Bearer " + token,
        "Content-Type": "application/json"
    };

    const response = await fetch(API_BASE + url, options);

    if (response.status === 401) {
        logout();
        return;
    }

    return response;
}