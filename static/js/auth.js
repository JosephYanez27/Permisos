// 🔹 Si ya hay token, redirigir al menú
document.addEventListener("DOMContentLoaded", () => {
    const token = localStorage.getItem("token");
    if (token) {
        window.location.href = "menu.html";
    }
});

// 🔹 Función Login
async function login() {

    const usuario = document.getElementById("usuario").value.trim();
    const password = document.getElementById("password").value.trim();

    if (!usuario || !password) {
        alert("Todos los campos son obligatorios");
        return;
    }

    // 🔐 Obtener token reCAPTCHA
    const recaptcha_token = grecaptcha.getResponse();

    if (!recaptcha_token) {
        alert("Por favor completa el reCAPTCHA");
        return;
    }

    try {

        const response = await fetch(`${API_URL.replace("/api", "")}/login`,  {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({
                usuario: usuario,
                password: password,
                recaptcha_token: recaptcha_token
            })
        });

        if (!response.ok) {

            const errorText = await response.text();
            alert(errorText || "Credenciales inválidas");

            grecaptcha.reset();
            return;
        }

        const data = await response.json();

        if (!data.token) {
            alert("Error al iniciar sesión");
            return;
        }

   // 🔹 Guardar token
localStorage.setItem("token", data.token);
localStorage.setItem("usuario",usuario);
localStorage.setItem("usuario", JSON.stringify({
    id: data.id,
    strnombreusuario: data.usuario
}));
// 🔥 OBTENER PERMISOS DESPUÉS DEL LOGIN
const resPermisos = await fetch(`${API_URL}/mis-permisos`, {
    headers: {
        "Authorization": "Bearer " + data.token
    }
});

if (!resPermisos.ok) {
    alert("Error obteniendo permisos");
    return;
}

const permisos = await resPermisos.json();

// 🔥 GUARDAR PERMISOS
localStorage.setItem("permisos", JSON.stringify(permisos));

// 🔹 Redirigir
window.location.href = "menu.html";

    } catch (error) {
        console.error("Error:", error);
        alert("Error de conexión con el servidor");
    }
}