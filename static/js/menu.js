async function cargarMenu(){

 const res = await fetchAuth("/menu");

 if(!res || !res.ok){
   console.error("Error cargando menú");
   return;
 }

 const data = await res.json();

 const menu = document.getElementById("menu-list");
 menu.innerHTML = "";

 data.forEach(padre => {

  let li = document.createElement("li");

  li.innerHTML = `
    <a href="#">${padre.nombre}</a>
  `;

  let ul = document.createElement("ul");
  ul.classList.add("submenu");

  padre.hijos.forEach(hijo => {

   ul.innerHTML += `
     <li>
       <a href="${hijo.nombre}.html">
         ${hijo.nombre}
       </a>
     </li>
   `;
  });

  li.appendChild(ul);
  menu.appendChild(li);
 });
}


async function aplicarPermisosAuto(){

 const res = await fetchAuth("/mis-permisos");

 if(!res) return;

 const data = await res.json();

 localStorage.setItem("permisos", JSON.stringify(data));

}
async function cargarLayout() {

    const res = await fetch("layout.html");
    const html = await res.text();

    document.getElementById("layout").innerHTML = html;

    // 🔥 después de cargar layout → cargar menú
    await cargarMenu();
}
async function cargarPerfilHeader() {

    const usuario = JSON.parse(localStorage.getItem("usuario"));
    if (!usuario) return;

    const saludo = document.getElementById("saludo");
    const img = document.getElementById("foto-user");

    // 🔥 si no existe → no hagas nada
    if (!saludo || !img) return;

    saludo.innerText = "Hola, " + usuario.strnombreusuario;

    try {
        const res = await fetchAuth(`/usuario/foto/${usuario.id}`);

        if (res && res.ok) {
            const data = await res.json();

            if (data.strfoto) {
                img.src = API_URL.replace("/api", "") + data.strfoto;
                return;
            }
        }

        // fallback
        img.src = `https://ui-avatars.com/api/?name=${usuario.strnombreusuario}`;

    } catch {
        img.src = `https://ui-avatars.com/api/?name=${usuario.strnombreusuario}`;
    }
}
function irPerfil() {
    window.location.href = "img.html";
}

document.addEventListener("DOMContentLoaded", async () => {
await cargarLayout();
 await  aplicarPermisosAuto();
   await cargarPerfilHeader();
   await cargarFoto();
});