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
  li.innerHTML = `<strong>${padre.nombre}</strong>`;

  let ul = document.createElement("ul");

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