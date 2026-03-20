let permisosGlobal = [];

async function aplicarPermisosAuto() {

    const response = await fetchAuth("/mis-permisos");
    if (!response || !response.ok) return;

    const data = await response.json();

    permisosGlobal = data;

    aplicarPermisosEnVista();
}
function tienePermiso(permiso) {

    const permisos = JSON.parse(localStorage.getItem("permisos")) || [];

    return permisos.some(p => p[permiso] === true);
}

function tienePermiso(tipo) {

    const modulo = document.body.dataset.modulo;

    const permiso = permisosGlobal.find(p => p.modulo === modulo);

    if (!permiso) return false;

    return permiso["bit" + tipo] === true;
}

function aplicarPermisosEnVista() {

    const elementos = document.querySelectorAll("[data-permiso]");

    elementos.forEach(el => {

        const tipo = el.dataset.permiso;

        if (!tienePermiso(tipo)) {
            el.style.display = "none";
        }
    });
}
function puedeEditarPermisos() {

    const modulo = document.body.dataset.modulo; // permisosperfil

    const permiso = permisosGlobal.find(p => p.modulo === modulo);

    if (!permiso) return false;

    return permiso.biteditar || permiso.bitagregar;
}
async function cargarLayout() {

    const res = await fetch("layout.html");
    const html = await res.text();

    document.getElementById("layout").innerHTML = html;

    // 🔥 después de cargar layout → cargar menú
    await cargarMenu();
}

window.aplicarPermisosAuto = aplicarPermisosAuto;