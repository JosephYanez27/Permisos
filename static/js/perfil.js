async function listarPerfiles() {

    const response = await fetchAuth("/perfil?page=1");

    if (!response) return;

    const data = await response.json();

    const tabla = document.getElementById("tabla-perfil");
    tabla.innerHTML = "";

    data.forEach(p => {

        tabla.innerHTML += `
            <tr>
                <td>${p.id}</td>
                <td>${p.strnombreperfil}</td>
                <td>${p.bitadministrador}</td>
                <td><button onclick="editarPerfil(${p.id}, '${p.strnombreperfil}', ${p.bitadministrador})">Editar</button></td>
                <td><button onclick="eliminarPerfil(${p.id})">Eliminar</button></td>
            </tr>
        `;
    });
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
}async function editarPerfil(id, nombreActual, adminActual) {

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