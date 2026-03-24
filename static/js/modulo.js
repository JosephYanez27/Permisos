let modulos = [];
let idEditar = null;

document.addEventListener("DOMContentLoaded",async () => {
    await aplicarPermisosAuto()
    cargarModulos();
});

async function cargarModulos(){

    const res = await fetchAuth("/modulo");
    const data = await res.json();

    modulos = data;

    renderTabla();
    cargarPadres();
}

function renderTabla(){

    const tabla = document.getElementById("tabla-modulos");
    tabla.innerHTML = "";

    modulos.forEach(m => {

        tabla.innerHTML += `
        <tr>

            <td>${m.strnombremodulo}</td>
        

            <td>
                <button data-permiso="editar" onclick="editar(${m.id})">
                    Editar
                </button>
            </td>

            <td>
                <button data-permiso="eliminar" onclick="eliminar(${m.id})">
                    Eliminar
                </button>
            </td>
        </tr>
        `;
    });

    // 🔥 CLAVE
    aplicarPermisosEnVista();
}

function cargarPadres(){

    const select = document.getElementById("padre");
    select.innerHTML = `<option value="">Sin padre</option>`;

    modulos.forEach(m => {
        select.innerHTML += `
            <option value="${m.id}">
                ${m.strnombremodulo}
            </option>
        `;
    });
}

function abrirModal(){
    idEditar = null;
    document.getElementById("modal").style.display = "block";
}

function cerrarModal(){
    document.getElementById("modal").style.display = "none";
}

async function guardar(){

    const nombre = document.getElementById("nombre").value.trim();
    const padre = document.getElementById("padre").value;

    // 🔐 VALIDACIÓN
    if (!validarModulo(nombre)) return;

    const data = {
        strnombremodulo: nombre,
        idmodulopadre: padre ? parseInt(padre) : null
    };

    const metodo = idEditar ? "PUT" : "POST";
    const url = idEditar ? `/modulo/${idEditar}` : "/modulo";

    try {

        const res = await fetchAuth(url, {
            method: metodo,
            body: JSON.stringify(data)
        });

        if (!res) return;

        const msg = await res.text();

        if (!res.ok) {
            alert("Error: " + msg);
            return;
        }

        alert(msg);

        cerrarModal();
        cargarModulos();

    } catch (error) {
        console.error("Error:", error);
        alert("Error de conexión");
    }
}

function editar(id){

    const m = modulos.find(x => x.id === id);

    if (!m) {
        alert("Módulo no encontrado");
        return;
    }

    idEditar = id;

    document.getElementById("nombre").value = m.strnombremodulo;
    document.getElementById("padre").value = m.idmodulopadre || "";

    abrirModal();
}

async function eliminar(id){

    if(!confirm("¿Eliminar módulo?")) return;

    const res = await fetchAuth(`/modulo/${id}`, {
        method: "DELETE"
    });

    alert(await res.text());
    cargarModulos();
}
function validarModulo(nombre) {

    if (!nombre) {
        alert("El nombre es obligatorio");
        return false;
    }

    if (nombre.length < 3 || nombre.length > 10) {
        alert("El nombre debe tener entre 3 y 10 caracteres");
        return false;
    }

    // 🔥 solo letras, números y espacios
    const regex = /^[a-zA-Z0-9áéíóúÁÉÍÓÚñÑ\s]+$/;

    if (!regex.test(nombre)) {
        alert("El nombre contiene caracteres inválidos");
        return false;
    }

    return true;
}