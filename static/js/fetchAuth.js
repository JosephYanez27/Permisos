

async function fetchAuth(url, options = {}) {

    const token = localStorage.getItem("token");

    if (!options.headers) {
        options.headers = {};
    }

    options.headers["Authorization"] = `Bearer ${token}`;
    options.headers["Content-Type"] = "application/json";

    const response = await fetch(API_URL + url, options);

    // 🔴 TOKEN EXPIRADO
    if (response.status === 401) {

        console.log("Token expirado");

        localStorage.removeItem("token");

        window.location.href = "login.html";

        return null;
    }

    return response;
}
window.fetchAuth = fetchAuth;