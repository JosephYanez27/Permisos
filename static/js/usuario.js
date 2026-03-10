let pagina = 1;
let idUsuarioEdicion = null;

// 🔹 Cargar perfiles para filtro y modal
document.addEventListener("DOMContentLoaded", async () => {

    // 🔹 Buscar mientras se escribe
    document.getElementById("filtro-usuario")
    .addEventListener("keyup", () => {
        pagina = 1;
        buscarUsuarios();
    });

    // 🔹 Filtro por perfil
    document.getElementById("filtro-perfil")
    .addEventListener("change", () => {
        pagina = 1;
        buscarUsuarios();
    });

    // 🔹 Filtro por estado
    document.getElementById("filtro-estado")
    .addEventListener("change", () => {
        pagina = 1;
        buscarUsuarios();
    });

    // 🔹 Primera carga
    buscarUsuarios();

});

async function cargarUsuarios(page = 1){

 const usuario = document.getElementById("usuario").value;

 const res = await fetchAuth(
   `/usuario?page=${page}&usuario=${usuario}`
 );

 const data = await res.json();

 pintarTabla(data);
}


let totalPaginas = 1;

async function buscarUsuarios(){

 const usuario = document.getElementById("filtro-usuario").value.trim();
 const perfil = document.getElementById("filtro-perfil").value;
 const estado = document.getElementById("filtro-estado").value;

 let query = `/usuario?page=${pagina}`;

 if(usuario) query += `&usuario=${encodeURIComponent(usuario)}`;
 if(perfil) query += `&perfil=${perfil}`;
 if(estado) query += `&estado=${estado}`;

 try{

  const res = await fetchAuth(query);

  if(res.status === 401){
   alert("Sesión expirada");
   window.location.href = "/login.html";
   return;
  }

  const result = await res.json();
  

  console.log("Respuesta API:", result);

  if(!result || !result.data){
   console.error("Respuesta inválida:", result);
   renderTabla([]);
   return;
  }

  renderTabla(result.data);

  if(result.total !== undefined){
   calcularPaginas(result.total);
  }

 }catch(error){
  console.error("Error cargando usuarios:", error);
 }

}

function renderTabla(data){
     console.log("Datos que llegan a la tabla:", data);

 const tabla = document.getElementById("tabla-usuarios");
 tabla.innerHTML = "";

 if(!Array.isArray(data)){
  console.error("Data no es arreglo:", data);
  return;
 }

 if(data.length === 0){
  tabla.innerHTML = `
   <tr>
    <td colspan="7">No se encontraron usuarios</td>
   </tr>
  `;
  return;
 }

 data.forEach(u => {

  tabla.innerHTML += `
  <tr>
   <td>${u.strnombreusuario ?? ""}</td>
  <td>${u.perfil ?? ""}</td>
<td>${u.estado ?? ""}</td>
   <td>${u.strcorreo ?? ""}</td>
   <td>${u.strnumerocelular ?? ""}</td>

   <td>
    <button onclick="editar(${u.id})">Editar</button>
   </td>

   <td>
    <button onclick="eliminar(${u.id})">Eliminar</button>
   </td>

  </tr>
  `;
 });

}
function calcularPaginas(total){

 const limit = 10;

 totalPaginas = Math.ceil(total / limit);

 let html = "";

 for(let i=1;i<=totalPaginas;i++){

  html += `<button onclick="irPagina(${i})">${i}</button>`;

 }

 document.getElementById("paginacion").innerHTML = html;
}

function irPagina(p){
 pagina = p;
 buscarUsuarios();
}



// 🔹 Eliminar
async function eliminar(id) {

    if (!confirm("¿Eliminar usuario?")) return;

    const response = await fetchAuth(`/usuario/${id}`, {
        method: "DELETE"
    });

    if (!response) return;

    alert("Usuario eliminado");
    buscarUsuarios();
}

// 🔹 Limpiar filtros
function limpiarFiltros() {

    document.getElementById("filtro-usuario").value = "";
    document.getElementById("filtro-perfil").value = "";
    document.getElementById("filtro-estado").value = "";

    pagina = 1;
    buscarUsuarios();
}

// 🔹 Abrir modal
function abrirModal() {

    idUsuarioEdicion = null;

    document.getElementById("usuario").value = "";
    document.getElementById("correo").value = "";
    document.getElementById("celular").value = "";

    document.getElementById("estado").value = "1"; // activo por defecto

    document.getElementById("modalUsuario").style.display = "block";
}

// 🔹 Cerrar modal
function cerrarModal() {
    document.getElementById("modalUsuario").style.display = "none";
}

// 🔹 Guardar usuario
async function guardarUsuario() {

    const usuarioData = {

        strnombreusuario: document.getElementById("usuario").value,

        idperfil: parseInt(
            document.getElementById("perfil").value
        ),

        strcorreo: document.getElementById("correo").value,

        strnumerocelular: document.getElementById("celular").value,

        idestadousuario: parseInt(
 document.getElementById("estado").value
)
    };

    const metodo = idUsuarioEdicion ? "PUT" : "POST";

    const endpoint = idUsuarioEdicion
        ? `/usuario/${idUsuarioEdicion}`
        : "/usuario";

    const response = await fetchAuth(endpoint, {
        method: metodo,
        body: JSON.stringify(usuarioData)
    });

    if (response && response.ok) {

        alert(
            idUsuarioEdicion
                ? "Usuario actualizado"
                : "Usuario creado"
        );

        cerrarModal();
        buscarUsuarios();

    } else {

        const error = await response.text();
        alert("Error: " + error);
    }
}

// 🔹 Editar
async function editar(id) {

    idUsuarioEdicion = id;

    const response = await fetchAuth(`/usuario/${id}`);
    if (!response || !response.ok) return;

    const u = await response.json();

    document.getElementById("usuario").value =
        u.strnombreusuario;

    document.getElementById("correo").value =
        u.strcorreo;

    document.getElementById("celular").value =
        u.strnumerocelular || "";

    document.getElementById("perfil").value =
        u.idperfil;

    document.getElementById("estado").value =
        u.idestado;

    document.getElementById("modalUsuario").style.display = "block";
}
fetch("/menu.html")
.then(res => res.text())
.then(html => {
    document.getElementById("menu").innerHTML = html;
});
// 🔹 Exponer funciones
window.buscarUsuarios = buscarUsuarios;
window.editar = editar;
window.eliminar = eliminar;
window.limpiarFiltros = limpiarFiltros;
window.guardarUsuario = guardarUsuario;
window.abrirModal = abrirModal;
window.cerrarModal = cerrarModal;
