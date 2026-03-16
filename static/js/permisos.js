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
                <td><input type="checkbox" ${p.bitagregar ? "checked" : ""} onchange="cambiar(${p.idmodulo}, 'bitagregar', this.checked)"></td>
                <td><input type="checkbox" ${p.biteditar ? "checked" : ""} onchange="cambiar(${p.idmodulo}, 'biteditar', this.checked)"></td>
                <td><input type="checkbox" ${p.biteliminar ? "checked" : ""} onchange="cambiar(${p.idmodulo}, 'biteliminar', this.checked)"></td>
                <td><input type="checkbox" ${p.bitconsulta ? "checked" : ""} onchange="cambiar(${p.idmodulo}, 'bitconsulta', this.checked)"></td>
                <td><input type="checkbox" ${p.bitdetalle ? "checked" : ""} onchange="cambiar(${p.idmodulo}, 'bitdetalle', this.checked)"></td>
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
permisosActuales.forEach(p => {

    if(
        p.bitagregar ||
        p.biteditar ||
        p.bitconsulta ||
        p.biteliminar ||
        p.bitdetalle
    ){
        // activar permiso consulta del padre
        let padre = permisosActuales.find(x => x.idmodulo === p.idpadre);

        if(padre){
            padre.bitconsulta = true;
        }
    }

});
function aplicarPermisosAuto(){

 const permisos = JSON.parse(localStorage.getItem("permisos"));

 if(!permisos) return;

 const modulo = document.body.dataset.modulo;

 const permisoModulo = permisos.find(p => p.modulo === modulo);

 if(!permisoModulo) return;

 document.querySelectorAll("[data-permiso]").forEach(el => {

  const tipo = el.dataset.permiso;

  if(!permisoModulo["bit"+tipo]){
   el.style.display = "none";
  }

 });

}

window.aplicarPermisosAuto = aplicarPermisosAuto;
