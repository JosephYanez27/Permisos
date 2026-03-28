let paginaActual = 1;
const registrosPorPagina = 5;
let filtroActual = "";

async function listarPerfiles() {

    filtroActual = document.getElementById("buscarPerfil")?.value || "";

    const response = await fetchAuth(`/perfil?page=${paginaActual}&filtro=${filtroActual}`);
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

    // 🔥 APLICAR PERMISOS (IMPORTANTE)
    aplicarPermisosEnVista();

    // 🔥 PAGINACIÓN
    renderPaginacion(data.length);
}
function buscarPerfiles(){
    paginaActual = 1;
    listarPerfiles();
}
function renderPaginacion(cantidad){

    const contenedor = document.getElementById("paginacion");
    contenedor.innerHTML = "";

    const esUltima = cantidad < registrosPorPagina;

    contenedor.innerHTML += `
        <button onclick="cambiarPagina(1)" ${paginaActual === 1 ? 'disabled' : ''}>⏮</button>
    `;

    contenedor.innerHTML += `
        <button onclick="cambiarPagina(${paginaActual - 1})" ${paginaActual === 1 ? 'disabled' : ''}>◀</button>
    `;

    contenedor.innerHTML += `
        <span>Página ${paginaActual}</span>
    `;

    contenedor.innerHTML += `
        <button onclick="cambiarPagina(${paginaActual + 1})" ${esUltima ? 'disabled' : ''}>▶</button>
    `;

    contenedor.innerHTML += `
        <button onclick="cambiarPagina(${paginaActual + 1})" ${esUltima ? 'disabled' : ''}>⏭</button>
    `;
}
function cambiarPagina(pagina){

    if(pagina < 1) return;

    paginaActual = pagina;
    listarPerfiles();
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
    await cargarLayout();
    await aplicarPermisosAuto(); // 🔥 PRIMERO permisos
    await listarPerfiles();      // 🔥 DESPUÉS render
    
});