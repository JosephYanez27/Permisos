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

async function buscarUsuarios() {
    // 1. Obtener valores de los inputs
    const usuarioInput = document.getElementById("filtro-usuario");
    const perfilInput = document.getElementById("filtro-perfil");
    const estadoInput = document.getElementById("filtro-estado");

    const usuario = usuarioInput ? usuarioInput.value : "";
    const perfil = perfilInput ? perfilInput.value : "";
    const estado = estadoInput ? estadoInput.value : "";

    // 2. DECLARAR la variable query (aquí es donde fallaba)
    let query = `/usuario?page=${pagina}`;

    // 3. Concatenar filtros
    if (usuario) query += `&usuario=${encodeURIComponent(usuario)}`;
    if (perfil) query += `&perfil=${perfil}`;
    if (estado) query += `&estado=${estado}`;

    console.log("Consultando a:", query); // Para depurar

    const response = await fetchAuth(query);
    if (!response) return;

    if (!response.ok) {
        const errorText = await response.text();
        console.error("Error en la petición:", errorText);
        return;
    }

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
let idUsuarioEdicion = null; // Para saber si editamos o creamos

// Función que llama el botón ➕ Agregar Usuario
function abrirModalUsuario() {
    idUsuarioEdicion = null;
    // Aquí deberías limpiar los campos de tu modal
    // Ejemplo: document.getElementById("form-usuario").reset();
    alert("Aquí deberías mostrar tu modal de registro");
    // Si usas un modal real de CSS/JS, cámbialo por:
    // document.getElementById("miModal").style.display = "block";
}

// Función para capturar los datos y enviar al servidor
async function guardarUsuario() {
    const usuarioData = {
        strnombreusuario: document.getElementById("nombre").value,
        idperfil: parseInt(document.getElementById("perfil").value),
        idestado: parseInt(document.getElementById("estado").value),
        strcorreo: document.getElementById("correo").value,
        strnumerocelular: document.getElementById("celular").value,
        // Agrega aquí el password si es nuevo
    };

    const metodo = idUsuarioEdicion ? "PUT" : "POST";
    const endpoint = idUsuarioEdicion ? `/usuario/${idUsuarioEdicion}` : "/usuario";

    const response = await fetchAuth(endpoint, {
        method: metodo,
        body: JSON.stringify(usuarioData)
    });

    if (response && response.ok) {
        alert(idUsuarioEdicion ? "Actualizado con éxito" : "Creado con éxito");
        // cerrarModal();
        buscarUsuarios();
    } else {
        const error = await response.text();
        alert("Error al guardar: " + error);
    }
}

// Función que se llama desde el botón "Editar" de la tabla
async function editar(id) {
    idUsuarioEdicion = id;
    
    // 1. Obtener los datos actuales del usuario
    const response = await fetchAuth(`/usuario/${id}`);
    if (!response || !response.ok) return;

    const u = await response.json();

    // 2. Llenar el modal con los datos
    // document.getElementById("nombre").value = u.strnombreusuario;
    // document.getElementById("perfil").value = u.idperfil;
    
    alert("Cargando datos del usuario " + id + " para editar...");
    // document.getElementById("miModal").style.display = "block";
}
function abrirModal(){
document.getElementById("modalUsuario").style.display="block";
}

function cerrarModal(){
document.getElementById("modalUsuario").style.display="none";
}

window.buscarUsuarios = buscarUsuarios;
window.abrirModalUsuario = abrirModalUsuario;
window.siguiente = siguiente;
window.anterior = anterior;
window.abrirModalUsuario = abrirModalUsuario;
window.guardarUsuario = guardarUsuario;
window.editar = editar;
window.limpiarFiltros = limpiarFiltros;
window.cerrarModal=cerrarModal;
window.abrirModal=abrirModal;