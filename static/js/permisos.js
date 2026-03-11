let permisosActuales = [];
let perfilSeleccionado = null;

// 🔹 Cargar perfiles al iniciar
document.addEventListener("DOMContentLoaded", async () => {
    await cargarPerfiles();
});

// 🔹 Cargar perfiles
async function cargarPerfiles() {

    const response = await fetchAuth("/perfil?page=1");
    if (!response) return;

    const data = await response.json();

    const select = document.getElementById("perfil-select");
    select.innerHTML = "";

    data.forEach(p => {
        select.innerHTML += `<option value="${p.id}">${p.strnombreperfil}</option>`;
    });
}

// 🔹 Buscar permisos del perfil
async function buscarPermisos() {

    perfilSeleccionado = document.getElementById("perfil-select").value;

    const response = await fetchAuth(`/permisosperfil/${perfilSeleccionado}`);
    if (!response) return;

    permisosActuales = await response.json();

    renderTabla();
}

// 🔹 Renderizar tabla
function renderTabla() {

    const tabla = document.getElementById("tabla-permisos");
    tabla.innerHTML = "";

    permisosActuales.forEach(p => {

        tabla.innerHTML += `
            <tr>
                <td>${p.modulo}</td>
                <td><input type="checkbox" ${p.bitagregar ? "checked" : ""} onchange="cambiar(${p.id}, 'bitagregar', this.checked)"></td>
                <td><input type="checkbox" ${p.biteditar ? "checked" : ""} onchange="cambiar(${p.id}, 'biteditar', this.checked)"></td>
                <td><input type="checkbox" ${p.biteliminar ? "checked" : ""} onchange="cambiar(${p.id}, 'biteliminar', this.checked)"></td>
                <td><input type="checkbox" ${p.bitconsulta ? "checked" : ""} onchange="cambiar(${p.id}, 'bitconsulta', this.checked)"></td>
                <td><input type="checkbox" ${p.bitdetalle ? "checked" : ""} onchange="cambiar(${p.id}, 'bitdetalle', this.checked)"></td>
            </tr>
        `;
    });
}

// 🔹 Cambiar valor en memoria
function cambiar(idmodulo, campo, valor) {

    const permiso = permisosActuales.find(p => p.idmodulo === idmodulo);

    if (permiso) {
        permiso[campo] = valor;
    }
}

// 🔹 Guardar permisos
async function guardarPermisos() {

    const response = await fetchAuth("/permisosperfil", {
        method: "PUT",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify(permisosActuales)
    });

    if (!response) return;

    alert("Permisos guardados correctamente");
}