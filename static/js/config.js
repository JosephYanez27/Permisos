const API_URL = "https://controlacceso-l9rs.onrender.com/api";

function getToken() {
    return localStorage.getItem("token");
}

function logout() {
    localStorage.removeItem("token");
    window.location.href = "login.html";
}
window.API_URL=API_URL;