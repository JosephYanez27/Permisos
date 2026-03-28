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

    // 🔥 PAGINACIÓN REAL
    const inicio = (paginaActual - 1) * registrosPorPagina;
    const fin = inicio + registrosPorPagina;

    const paginaData = modulosFiltrados.slice(inicio, fin);

    if(paginaData.length === 0){
        tabla.innerHTML = `<tr><td colspan="3">Sin resultados</td></tr>`;
        renderPaginacion();
        return;
    }

    paginaData.forEach(m => {

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

    aplicarPermisosEnVista();

    // 🔥 IMPORTANTE
    renderPaginacion();
}

function cargarPadres(){

    const select = document.getElementById("padre");
    select.innerHTML = `<option value="">Sin padre</option>`;

    // 🔥 SOLO PADRES (sin idmodulopadre)
    const padres = modulos.filter(m => !m.idmodulopadre);

    padres.forEach(m => {
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
let paginaActual = 1;
const registrosPorPagina = 5;
let modulosFiltrados = [];
function filtrarModulos(){

    const texto = document.getElementById("buscarModulo").value.toLowerCase();

    modulosFiltrados = modulos.filter(m =>
        m.strnombremodulo.toLowerCase().includes(texto)
    );

    paginaActual = 1;

    renderTabla();
}
async function cargarModulos(){

    const res = await fetchAuth("/modulo");
    const data = await res.json();

    modulos = data;
    modulosFiltrados = data; // 🔥 importante

    renderTabla();
    cargarPadres();
}
function renderPaginacion(){

    const totalPaginas = Math.ceil(modulosFiltrados.length / registrosPorPagina);
    const contenedor = document.getElementById("paginacion");

    contenedor.innerHTML = "";

    if(totalPaginas === 0) return;

    // ⏮️ INICIO
    contenedor.innerHTML += `
        <button onclick="cambiarPagina(1)" ${paginaActual === 1 ? 'disabled' : ''}>
            ⏮
        </button>
    `;

    // ◀️ ANTERIOR
    contenedor.innerHTML += `
        <button onclick="cambiarPagina(${paginaActual - 1})"
            ${paginaActual === 1 ? 'disabled' : ''}>
            ◀
        </button>
    `;

    // 🔢 NÚMEROS (máx 5 visibles)
    let inicio = Math.max(1, paginaActual - 2);
    let fin = Math.min(totalPaginas, paginaActual + 2);

    for(let i = inicio; i <= fin; i++){
        contenedor.innerHTML += `
            <button onclick="cambiarPagina(${i})"
                class="${i === paginaActual ? 'activo' : ''}">
                ${i}
            </button>
        `;
    }

    // ▶️ SIGUIENTE
    contenedor.innerHTML += `
        <button onclick="cambiarPagina(${paginaActual + 1})"
            ${paginaActual === totalPaginas ? 'disabled' : ''}>
            ▶
        </button>
    `;

    // ⏭️ FINAL
    contenedor.innerHTML += `
        <button onclick="cambiarPagina(${totalPaginas})"
            ${paginaActual === totalPaginas ? 'disabled' : ''}>
            ⏭
        </button>
    `;
}
function cambiarPagina(pagina){

    const totalPaginas = Math.ceil(modulosFiltrados.length / registrosPorPagina);

    if(pagina < 1 || pagina > totalPaginas) return;

    paginaActual = pagina;

    renderTabla();
}