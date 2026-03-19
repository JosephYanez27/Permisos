async function listarPerfiles() {

    const response = await fetchAuth("/perfil?page=1");
    if (!response) return;

    const data = await response.json();

    const tabla = document.getElementById("tabla-perfil");
    tabla.innerHTML = "";

    data.forEach(p => {

        tabla.innerHTML += `
            <tr>
                <td>${p.id}</td>
                <td>${p.strnombreperfil}</td>
                <td>${p.bitadministrador}</td>

                <td>
                  <button data-permiso="editar"
                    onclick="editarPerfil(${p.id}, '${p.strnombreperfil}', ${p.bitadministrador})">
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
let idPerfilEditando = null;

// 🔹 Abrir modal nuevo
function abrirModalPerfil() {
    idPerfilEditando = null;

    document.getElementById("tituloModal").innerText = "Nuevo Perfil";
    document.getElementById("nombrePerfil").value = "";
    document.getElementById("adminPerfil").checked = false;

    document.getElementById("modalPerfil").style.display = "block";
}

// 🔹 Abrir modal editar
function editarPerfil(id, nombreActual, adminActual) {
    idPerfilEditando = id;

    document.getElementById("tituloModal").innerText = "Editar Perfil";
    document.getElementById("nombrePerfil").value = nombreActual;
    document.getElementById("adminPerfil").checked = adminActual;

    document.getElementById("modalPerfil").style.display = "block";
}

// 🔹 Cerrar modal
function cerrarModalPerfil() {
    document.getElementById("modalPerfil").style.display = "none";
}

// 🔹 Guardar
async function guardarPerfil() {

    const nombre = document.getElementById("nombrePerfil").value.trim();
    const admin = document.getElementById("adminPerfil").checked;

    if (!nombre) {
        alert("Nombre requerido");
        return;
    }

    const endpoint = idPerfilEditando
        ? `/perfil/${idPerfilEditando}`
        : "/perfil";

    const metodo = idPerfilEditando ? "PUT" : "POST";

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

async function eliminarPerfil(id) {

    if (!confirm("¿Eliminar perfil?")) return;

    const response = await fetchAuth(`/perfil/${id}`, {
        method: "DELETE"
    });

    if (!response) return;

    alert(await response.text());
    listarPerfiles();
}
