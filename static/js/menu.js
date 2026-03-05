async function cargarMenu() {

    const response = await fetchAuth("/api/mis-permisos");
    if (!response) return;

    const permisos = await response.json();

    const menu = document.getElementById("menu-list");
    menu.innerHTML = "";

    const grupos = {};

    // Agrupar por menú
    permisos.forEach(p => {

        const padre = p.menu || "General";

        if (!grupos[padre]) {
            grupos[padre] = [];
        }

        grupos[padre].push(p);
    });

    // Crear menú
    for (const padre in grupos) {

        const liPadre = document.createElement("li");
        liPadre.innerHTML = `<strong>${padre}</strong>`;

        const ulHijos = document.createElement("ul");

        grupos[padre].forEach(m => {

            const modulo = m.modulo.toLowerCase();

            ulHijos.innerHTML += `
                <li>
                    <a href="${modulo}.html">${m.modulo}</a>
                </li>
            `;
        });

        liPadre.appendChild(ulHijos);
        menu.appendChild(liPadre);
    }
}

cargarMenu();