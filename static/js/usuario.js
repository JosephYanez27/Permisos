let pagina = 1;

// 🔹 Cargar perfiles para filtro
document.addEventListener("DOMContentLoaded", async () => {
    await cargarPerfilesFiltro();
    buscarUsuarios();
});

async function cargarPerfilesFiltro() {

    const response = await fetchAuth("/perfil?page=1");
    if (!response) return;

    const data = await response.json();

    const select = document.getElementById("filtro-perfil");

    data.forEach(p => {
        select.innerHTML += `<option value="${p.id}">${p.strnombreperfil}</option>`;
    });
}

// 🔹 Buscar usuarios con filtros
async function buscarUsuarios() {

    const usuario = document.getElementById("filtro-usuario").value;
    const perfil = document.getElementById("filtro-perfil").value;
    const estado = document.getElementById("filtro-estado").value;

    let query = `/usuario?page=${pagina}`;

    if (usuario) query += `&usuario=${usuario}`;
    if (perfil) query += `&perfil=${perfil}`;
    if (estado) query += `&estado=${estado}`;

    const response = await fetchAuth(query);
    if (!response) return;

    const data = await response.json();

    renderTabla(data);
}

// 🔹 Render tabla
function renderTabla(data) {

    const tabla = document.getElementById("tabla-usuarios");
    tabla.innerHTML = "";

    data.forEach(u => {

        tabla.innerHTML += `
            <tr>
                <td>${u.strnombreusuario}</td>
                <td>${u.perfil}</td>
                <td>${u.estado}</td>
                <td>${u.strcorreo}</td>
                <td>${u.strnumerocelular}</td>
                <td><button onclick="editar(${u.id})">Editar</button></td>
                <td><button onclick="eliminar(${u.id})">Eliminar</button></td>
            </tr>
        `;
    });

    document.getElementById("pagina-actual").innerText = pagina;
}

// 🔹 Paginación
function siguiente() {
    pagina++;
    buscarUsuarios();
}

function anterior() {
    if (pagina > 1) {
        pagina--;
        buscarUsuarios();
    }
}

// 🔹 Eliminar usuario
async function eliminar(id) {

    if (!confirm("¿Eliminar usuario?")) return;

    const response = await fetchAuth(`/usuario/${id}`, {
        method: "DELETE"
    });

    if (!response) return;

    alert("Usuario eliminado");
    buscarUsuarios();
}

// 🔹 Limpiar filtros
function limpiarFiltros() {
    document.getElementById("filtro-usuario").value = "";
    document.getElementById("filtro-perfil").value = "";
    document.getElementById("filtro-estado").value = "";
    pagina = 1;
    buscarUsuarios();
}