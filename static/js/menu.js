async function cargarMenu() {

    const response = await fetchAuth("/mis-permisos");
    if (!response) return;

    const permisos = await response.json();

    const menu = document.getElementById("menu-list");
    menu.innerHTML = "";

    const grupos = {};

    permisos.forEach(p => {

        const padre = p.menu || "General";

        if (!grupos[padre]) {
            grupos[padre] = [];
        }

        grupos[padre].push(p);
    });

    for (const padre in grupos) {

        const liPadre = document.createElement("li");

        const titulo = document.createElement("div");
        titulo.className = "menu-titulo";
        titulo.textContent = padre;

        const ulHijos = document.createElement("ul");
        ulHijos.className = "submenu";

        grupos[padre].forEach(m => {

            const li = document.createElement("li");

            li.innerHTML = `
                <a href="${m.modulo.toLowerCase()}.html">
                    ${m.modulo}
                </a>
            `;

            ulHijos.appendChild(li);
        });

        // Toggle submenu
        titulo.addEventListener("click", () => {
            ulHijos.classList.toggle("activo");
        });

        liPadre.appendChild(titulo);
        liPadre.appendChild(ulHijos);

        menu.appendChild(liPadre);
    }
}

cargarMenu();