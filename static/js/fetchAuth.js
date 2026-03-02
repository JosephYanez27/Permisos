const API_BASE = "/api";

function fetchAuth(endpoint, options = {}) {

    const token = localStorage.getItem("token");

    if (!token) {
        window.location.href = "login.html";
        return null;
    }

    return fetch(API_BASE + endpoint, {   // 🔥 AQUÍ SE CONCATENA
        ...options,
        headers: {
            "Content-Type": "application/json",
            "Authorization": "Bearer " + token,
            ...options.headers
        }
    });
}