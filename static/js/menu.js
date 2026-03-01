async function cargarMenu() {

    const response = await fetchAuth("/mis-permisos");

    if (!response) return;

    const permisos = await response.json();

    const menu = document.getElementById("menu-list");
    menu.innerHTML = "";

    permisos.forEach(p => {

        const li = document.createElement("li");
        const modulo = p.modulo.toLowerCase();

        li.innerHTML = `<a href="${modulo}.html">${p.modulo}</a>`;
        menu.appendChild(li);
    });
}

cargarMenu();