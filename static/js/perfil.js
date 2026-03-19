async function listarPerfiles() {

    const response = await fetchAuth("/perfil?page=1");
    if (!response) return;

    const data = await response.json();

    const tabla = document.getElementById("tabla-perfil");
    tabla.innerHTML = "";

    data.forEach(p => {

      tabla.innerHTML += `
    <tr>
        <td>${p.strnombreperfil}</td>
        <td>${p.bitadministrador ? "Sí" : "No"}</td>

        <td>
          <button data-permiso="editar"
            onclick="abrirModalPerfil(${p.id}, '${p.strnombreperfil}', ${p.bitadministrador})">
            Editar
          </button>
        </td>

        <td>
          <button data-permiso="eliminar"
            onclick="eliminarPerfil(${p.id})">
            Eliminar
          </button>
        </td>
    </tr>
`;
    });

    // 🔥 AQUI TAMBIÉN
    aplicarPermisosEnVista();
}
async function agregarPerfil() {

    const nombre = prompt("Nombre del perfil:");
    if (!nombre) return;

    const admin = confirm("¿Es administrador?");

    const response = await fetchAuth("/perfil", {
        method: "POST",
        body: JSON.stringify({
            strnombreperfil: nombre,
            bitadministrador: admin
        })
    });

    if (!response) return;

    const text = await response.text();
    alert(text);

    listarPerfiles();
}
async function editarPerfil(id, nombreActual, adminActual) {

    const nombre = prompt("Nuevo nombre:", nombreActual);
    if (!nombre) return;

    const admin = confirm("¿Administrador?");

    const response = await fetchAuth(`/perfil/${id}`, {
        method: "PUT",
        body: JSON.stringify({
            strnombreperfil: nombre,
            bitadministrador: admin
        })
    });

    if (!response) return;

    alert(await response.text());
    listarPerfiles();
}

async function eliminarPerfil(id) {

    if (!confirm("¿Eliminar perfil?")) return;

    const response = await fetchAuth(`/perfil/${id}`, {
        method: "DELETE"
    });

    if (!response) return;

    alert(await response.text());
    listarPerfiles();
}
let idPerfilEditando = null;

function abrirModalPerfil(id = null, nombre = "", admin = false) {

    idPerfilEditando = id;

    document.getElementById("modalPerfil").style.display = "block";

    document.getElementById("nombrePerfil").value = nombre;
    document.getElementById("adminPerfil").checked = admin;

    document.getElementById("tituloModal").innerText =
        id ? "Editar Perfil" : "Nuevo Perfil";
}

function cerrarModalPerfil() {
    document.getElementById("modalPerfil").style.display = "none";
}
async function guardarPerfil() {

    const nombre = document.getElementById("nombrePerfil").value.trim();
    const admin = document.getElementById("adminPerfil").checked;

    if (!nombre) {
        alert("Nombre requerido");
        return;
    }

    const metodo = idPerfilEditando ? "PUT" : "POST";
    const endpoint = idPerfilEditando
        ? `/perfil/${idPerfilEditando}`
        : "/perfil";

    const response = await fetchAuth(endpoint, {
        method: metodo,
        body: JSON.stringify({
            strnombreperfil: nombre,
            bitadministrador: admin
        })
    });

    if (!response) return;

    alert(await response.text());

    cerrarModalPerfil();
    listarPerfiles();
}
document.addEventListener("DOMContentLoaded", async () => {

    await aplicarPermisosAuto(); // 🔥 PRIMERO permisos
    await listarPerfiles();      // 🔥 DESPUÉS render

});