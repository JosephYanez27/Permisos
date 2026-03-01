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
                <td><button>Editar</button></td>
                <td><button>Eliminar</button></td>
            </tr>
        `;
    });
}