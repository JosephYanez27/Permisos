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

function mostrarUsuario(){
    const nombre = localStorage.getItem("usuario") || "Usuario";
    document.getElementById("saludo").innerText = "Hola, " + nombre;
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

document.addEventListener("DOMContentLoaded", async () => {
await cargarLayout();
 await  aplicarPermisosAuto();
    mostrarUsuario();

});