let pagina = 1;
let idUsuarioEdicion = null;

// 🔹 Cargar perfiles para filtro y modal
document.addEventListener("DOMContentLoaded", async () => {
    await cargarPerfiles();
    buscarUsuarios();
});

async function cargarPerfiles() {

    const response = await fetchAuth("/perfil?page=1");
    if (!response) return;

    const data = await response.json();

    const filtro = document.getElementById("filtro-perfil");
    const modal = document.getElementById("perfil");

    filtro.innerHTML = `<option value="">Todos</option>`;

    data.forEach(p => {

        filtro.innerHTML += `<option value="${p.id}">
            ${p.strnombreperfil}
        </option>`;

        modal.innerHTML += `<option value="${p.id}">
            ${p.strnombreperfil}
        </option>`;
    });
}

async function buscarUsuarios() {

    const usuario = document.getElementById("filtro-usuario").value;
    const perfil = document.getElementById("filtro-perfil").value;
    const estado = document.getElementById("filtro-estado").value;

    let query = `/usuario?page=${pagina}`;

    if (usuario) query += `&usuario=${encodeURIComponent(usuario)}`;
    if (perfil) query += `&perfil=${perfil}`;
    if (estado) query += `&estado=${estado}`;

    const response = await fetchAuth(query);
    if (!response || !response.ok) return;

    const data = await response.json();
    renderTabla(data);
}

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
            <td>${u.strnumerocelular || ""}</td>

            <td>
                <button onclick="editar(${u.id})">
                Editar
                </button>
            </td>

            <td>
                <button onclick="eliminar(${u.id})">
                Eliminar
                </button>
            </td>
        </tr>
        `;
    });

    document.getElementById("pagina-actual").innerText = pagina;
}

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

// 🔹 Eliminar
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

// 🔹 Abrir modal
function abrirModal() {

    idUsuarioEdicion = null;

    document.getElementById("usuario").value = "";
    document.getElementById("correo").value = "";
    document.getElementById("celular").value = "";

    document.getElementById("estado").value = "1"; // activo por defecto

    document.getElementById("modalUsuario").style.display = "block";
}

// 🔹 Cerrar modal
function cerrarModal() {
    document.getElementById("modalUsuario").style.display = "none";
}

// 🔹 Guardar usuario
async function guardarUsuario() {

    const usuarioData = {

        strnombreusuario: document.getElementById("usuario").value,

        idperfil: parseInt(
            document.getElementById("perfil").value
        ),

        strcorreo: document.getElementById("correo").value,

        strnumerocelular: document.getElementById("celular").value,

        idestado: parseInt(
            document.getElementById("estado").value
        )
    };

    const metodo = idUsuarioEdicion ? "PUT" : "POST";

    const endpoint = idUsuarioEdicion
        ? `/usuario/${idUsuarioEdicion}`
        : "/usuario";

    const response = await fetchAuth(endpoint, {
        method: metodo,
        body: JSON.stringify(usuarioData)
    });

    if (response && response.ok) {

        alert(
            idUsuarioEdicion
                ? "Usuario actualizado"
                : "Usuario creado"
        );

        cerrarModal();
        buscarUsuarios();

    } else {

        const error = await response.text();
        alert("Error: " + error);
    }
}

// 🔹 Editar
async function editar(id) {

    idUsuarioEdicion = id;

    const response = await fetchAuth(`/usuario/${id}`);
    if (!response || !response.ok) return;

    const u = await response.json();

    document.getElementById("usuario").value =
        u.strnombreusuario;

    document.getElementById("correo").value =
        u.strcorreo;

    document.getElementById("celular").value =
        u.strnumerocelular || "";

    document.getElementById("perfil").value =
        u.idperfil;

    document.getElementById("estado").value =
        u.idestado;

    document.getElementById("modalUsuario").style.display = "block";
}

// 🔹 Exponer funciones
window.buscarUsuarios = buscarUsuarios;
window.siguiente = siguiente;
window.anterior = anterior;
window.editar = editar;
window.eliminar = eliminar;
window.limpiarFiltros = limpiarFiltros;
window.guardarUsuario = guardarUsuario;
window.abrirModal = abrirModal;
window.cerrarModal = cerrarModal;