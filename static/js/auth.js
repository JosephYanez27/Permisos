// 🔹 Si ya hay token, redirigir al menú
document.addEventListener("DOMContentLoaded", () => {
    const token = localStorage.getItem("token");
    if (token) {
        window.location.href = "menu.html";
    }
});

async function login() {

    const usuario = document.getElementById("usuario").value.trim();
    const password = document.getElementById("password").value.trim();

    if (!usuario || !password) {
        alert("Todos los campos son obligatorios");
        return;
    }

    // 🔥 AQUÍ VA
    if (!validarLogin()) return;

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

        localStorage.setItem("usuario", JSON.stringify({
            id: data.id,
            strnombreusuario: data.usuario
        }));

        // 🔥 PERMISOS
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

        localStorage.setItem("permisos", JSON.stringify(permisos));

        window.location.href = "menu.html";

    } catch (error) {
        console.error("Error:", error);
        alert("Error de conexión con el servidor");
    }
}

function validarLogin() {

    const usuario = document.getElementById("usuario").value.trim();
    const password = document.getElementById("password").value.trim();

    if (usuario.length < 3 || usuario.length > 20) {
        alert("El usuario debe tener entre 3 y 20 caracteres");
        return false;
    }

    if (password.length < 6 || password.length > 30) {
        alert("La contraseña debe tener entre 6 y 30 caracteres");
        return false;
    }

    // 🔥 evitar caracteres peligrosos básicos
    const regex = /^[a-zA-Z0-9_@.]+$/;

    if (!regex.test(usuario)) {
        alert("Usuario inválido");
        return false;
    }

    return true;
}