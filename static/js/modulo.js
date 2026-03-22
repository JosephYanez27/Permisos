let modulos = [];
let idEditar = null;

document.addEventListener("DOMContentLoaded", () => {
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
            <td>${m.id}</td>
            <td>${m.strnombremodulo}</td>
            <td>${m.idmodulopadre ?? "-"}</td>

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

    const data = {
        strnombremodulo: document.getElementById("nombre").value,
        idmodulopadre: document.getElementById("padre").value || null
    };

    const metodo = idEditar ? "PUT" : "POST";
    const url = idEditar ? `/modulo/${idEditar}` : "/modulo";

    const res = await fetchAuth(url, {
        method: metodo,
        body: JSON.stringify(data)
    });

    alert(await res.text());

    cerrarModal();
    cargarModulos();
}

function editar(id){

    const m = modulos.find(x => x.id === id);

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